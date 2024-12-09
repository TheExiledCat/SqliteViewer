using NStack;
using Terminal.Gui;
using Terminal.Gui.Trees;

public class DatabaseViewerLayout : Window
{
    DatabaseTreeView databaseWindow;
    Window databaseResultsWindow;
    Window logWindow;
    public ustring FilePath { get; init; } = "";

    public DatabaseViewerLayout(ustring filepath)
    {
        FilePath = filepath;

        databaseWindow = new DatabaseTreeView(FilePath)
        {
            X = 0,
            Y = 0,
            Width = Dim.Percent(25),
            Height = Dim.Percent(50),
        };
        databaseWindow.OnSelect += (w) =>
        {
            w.X = Pos.Right(databaseWindow);
            w.Y = 0;
            Remove(databaseResultsWindow);

            databaseResultsWindow = w;
            Add(databaseResultsWindow);
        };
        databaseResultsWindow = new Window("Results")
        {
            X = Pos.Right(databaseWindow),
            Y = 0,
            Width = Dim.Fill(),
            Height = Dim.Fill(),
        };
        logWindow = new Window("Log")
        {
            X = 0,
            Y = Pos.Bottom(databaseWindow),
            Width = Dim.Percent(25),
            Height = Dim.Percent(50),
        };

        Add(databaseWindow, logWindow, databaseResultsWindow);
    }
}
