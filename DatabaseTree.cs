using Terminal.Gui;
using Terminal.Gui.Trees;

public class DatabaseTreeView : Window
{
    TreeView m_TableTree;
    public event Action<Window> OnSelect;

    public DatabaseTreeView()
    {
        m_TableTree = new TreeView
        {
            X = 0,
            Y = 0,
            Width = Dim.Fill(),
            Height = Dim.Fill(),
            ObjectActivationKey = Key.Enter,
        };
        Func<ITreeNode, bool> isLeaf = (n) =>
        {
            return n.Children.Count == 0;
        };

        TreeNode tableNode = new TreeNode("Tables");
        string[] fakeTables = ["Users", "Admins", "Stations", "Cars"];
        TreeNode[] fakeTableNodes = fakeTables.Select(n => new TreeNode(n)).ToArray();
        tableNode.Children = fakeTableNodes;
        m_TableTree.ObjectActivated += (e) =>
        {
            if (isLeaf(e.ActivatedObject))
            {
                OnSelect?.Invoke(new Window());
            }
            else if (e.ActivatedObject == tableNode)
            {
                //handle logic for custom queries
            }
        };
        m_TableTree.AddObject(tableNode);
    }
}
