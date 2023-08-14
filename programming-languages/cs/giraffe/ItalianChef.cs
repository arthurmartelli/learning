namespace giraffe
{
    class ItalianChef : Chef
    {
        public ItalianChef()
        {
        }

        public void MakeSpaghetti()
        {
            Console.WriteLine("The Chef makes spaghetti");
        }

        public override void MakeSpecialDish()
        {
            Console.WriteLine("The Chef makes Chicken Parmesan");
        }
    }
}