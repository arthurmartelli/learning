namespace giraffe
{
    class Book
    {
        public string title;
        public string author;
        public int pages;

        public Book(string title, string author, int pages)
        {
            this.title = title;
            this.author = author;
            this.pages = pages;

            Console.WriteLine("Creating book: {0}", title);
        }
    }
}