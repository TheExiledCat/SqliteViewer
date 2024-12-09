using System;
using System.Collections.Generic;
using System.Data;
using Microsoft.Data.Sqlite;

public static class SQLiteExtensions
{
    /// <summary>
    /// Retrieves the column names and their SQL data types for the specified table.
    /// </summary>
    /// <param name="connection">An open SQLiteConnection instance.</param>
    /// <param name="tableName">The name of the table.</param>
    /// <returns>A dictionary with column names as keys and their SQL data types as values.</returns>
    public static Dictionary<string, string> GetTableSchema(
        this SqliteConnection connection,
        string tableName
    )
    {
        if (connection.State != System.Data.ConnectionState.Open)
        {
            throw new InvalidOperationException("The SQLite connection must be open.");
        }

        var schema = new Dictionary<string, string>();
        string query = $"PRAGMA table_info({tableName});";

        using (var command = new SqliteCommand(query, connection))
        using (var reader = command.ExecuteReader())
        {
            while (reader.Read())
            {
                string columnName = reader["name"].ToString();
                string dataType = reader["type"].ToString();
                schema[columnName] = dataType;
            }
        }

        return schema;
    }

    /// <summary>
    /// Converts a SQLiteDataReader to a DataTable.
    /// </summary>
    /// <param name="reader">The SQLiteDataReader instance.</param>
    /// <returns>A DataTable containing all rows from the query result.</returns>
    public static DataTable ToDataTable(this IDataReader reader)
    {
        var dataTable = new DataTable();

        // Load column schema and create table structure
        for (int i = 0; i < reader.FieldCount; i++)
        {
            var columnName = reader.GetName(i);
            var columnType = reader.GetFieldType(i);
            dataTable.Columns.Add(columnName, columnType ?? typeof(string)); // Default to string if type is null
        }

        // Populate rows
        while (reader.Read())
        {
            var row = dataTable.NewRow();
            for (int i = 0; i < reader.FieldCount; i++)
            {
                row[i] = reader.IsDBNull(i) ? DBNull.Value : reader.GetValue(i);
            }
            dataTable.Rows.Add(row);
        }

        return dataTable;
    }

    /// <summary>
    /// Retrieves the names of all tables in the connected SQLite database, including system tables.
    /// </summary>
    /// <param name="connection">An open SQLiteConnection instance.</param>
    /// <returns>A list of table names in the database.</returns>
    public static List<string> GetAllTableNames(this SqliteConnection connection)
    {
        if (connection.State != System.Data.ConnectionState.Open)
        {
            throw new InvalidOperationException("The SQLite connection must be open.");
        }

        var tableNames = new List<string>();
        const string query = "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name";

        using (var command = new SqliteCommand(query, connection))
        using (var reader = command.ExecuteReader())
        {
            while (reader.Read())
            {
                tableNames.Add(reader.GetString(0)); // First column contains the table name
            }
        }

        return tableNames;
    }

    /// <summary>
    /// Converts a database value to its corresponding CLR type.
    /// </summary>
    /// <param name="value">The database value.</param>
    /// <returns>The value in its CLR type or null if DBNull.</returns>
    private static object ConvertToClrType(object value)
    {
        if (value == DBNull.Value)
        {
            return null;
        }

        return value switch
        {
            long l => l, // SQLite stores integers as Int64
            double d => d, // SQLite stores real numbers as Double
            string s => s, // Strings remain as strings
            byte[] b => b, // BLOBs are stored as byte arrays
            bool b => b, // SQLite boolean is often treated as integers
            DateTime dt => dt, // SQLite DateTime conversion
            _ => value // Fallback for other types
        };
    }
}
