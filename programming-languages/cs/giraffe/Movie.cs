namespace giraffe
{
    internal class Movie
    {

        public string title = "";
        public string director = "";
        private string rating = "";

        public Movie(string title, string director, string rating)
        {
            this.title = title;
            this.director = director;
            this.Rating = rating;
        }

        public string Rating
        {
            // implements getter and setter
            // getter just returns the value
            // and setter checks if the rating
            // is valid, and if not, sets it to
            // NR
            get => rating;
            set
            {
                string[] possible_ratings = { "G", "PG", "PG-13", "R", "NR" };
                if (possible_ratings.Contains(value))
                {
                    rating = value;
                    return;
                }

                rating = "NR";
            }
        }
    }


}