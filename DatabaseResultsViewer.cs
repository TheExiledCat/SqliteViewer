using System;
using System.Data;
using Microsoft.Data.Sqlite;
using Terminal.Gui;

public class DatabaseResultsViewer : Window
{
    SqliteConnection m_Connection;
    TextView m_QueryInput;
    TableView tableView;

    public DatabaseResultsViewer(SqliteConnection connection)
    {
        m_Connection = connection;
        m_QueryInput = new TextView
        {
            X = 0,
            Y = 0,
            Width = Dim.Fill(),
            Height = Dim.Percent(20)
        };
        tableView = new TableView()
        {
            X = 0,
            Y = Pos.Bottom(m_QueryInput),
            Width = Dim.Fill(),
            Height = Dim.Fill(),
        };
        Add(m_QueryInput);
        Add(tableView);
    }

    public void Load(DataTable dataTable)
    {
        DataTable table = dataTable;

        tableView.Table = table;
    }

    public void RunQuery(SqliteCommand command)
    {
        m_QueryInput.Text = command.CommandText;
        Load(command.ExecuteReader().ToDataTable());
    }
}
