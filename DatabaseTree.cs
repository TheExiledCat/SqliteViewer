using Microsoft.Data.Sqlite;
using NStack;
using Terminal.Gui;
using Terminal.Gui.Trees;

public class DatabaseTreeView : Window
{
    TreeView m_TableTree;
    public event Action<Window> OnSelect;
    ustring m_DatabasePath;
    SqliteConnection m_Connection;

    public DatabaseTreeView(ustring databaseFilePath)
    {
        m_DatabasePath = databaseFilePath;
        m_Connection = new SqliteConnection($"Data Source={m_DatabasePath}");
        m_Connection.Open();
        Refresh();
    }

    public void Refresh()
    {
        m_TableTree = new TreeView
        {
            X = 0,
            Y = 0,
            Width = Dim.Fill(),
            Height = Dim.Fill(),
        };
        Title =
            $"{Path.GetFileName((string)m_DatabasePath)} ({m_TableTree.ObjectActivationKey} to select)";
        Func<ITreeNode, bool> isLeaf = (n) =>
        {
            return n.Children.Count == 0;
        };

        TreeNode tableNode = new DataFrameTreeNode("Tables");
        string[] tables = m_Connection.GetAllTableNames().ToArray();
        TreeNode[] tableNodes = tables.Select(n => new TreeNode(n)).ToArray();
        tableNode.Children = tableNodes;
        m_TableTree.ObjectActivated += (e) =>
        {
            if (isLeaf(e.ActivatedObject))
            {
                SqliteCommand command = m_Connection.CreateCommand();
                command.CommandText = $"SELECT * FROM {e.ActivatedObject.Text}";

                DatabaseResultsViewer databaseResultsViewer = new DatabaseResultsViewer(
                    m_Connection
                )
                {
                    Title = $"Results: {e.ActivatedObject.Text}",
                    Height = Dim.Fill()
                };
                databaseResultsViewer.RunQuery(command);
                OnSelect?.Invoke(databaseResultsViewer);
            }
            else if (e.ActivatedObject == tableNode)
            {
                //handle logic for custom queries
            }
        };
        m_TableTree.AddObject(tableNode);
        Add(m_TableTree);
    }
}
