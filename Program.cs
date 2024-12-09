namespace SqliteViewer;

using System.Configuration;
using NStack;
using Terminal.Gui;

class Program
{
    static void Main(string[] args)
    {
        Application.Init();
        Colors.Base = Colors.Menu;
        Label selected = new Label();

        OpenDialog dialog = new OpenDialog(
            "Open the Sqlite encoded file",
            "",
            openMode: OpenDialog.OpenMode.File
        )
        {
            AllowsMultipleSelection = false,
        };
        Application.Run(dialog);
        ustring file = dialog.FilePath;
        selected.Text = file;
        DatabaseViewerLayout databaseViewer = new DatabaseViewerLayout(file);

        Application.Run(databaseViewer);
        Application.Shutdown();
    }
}
