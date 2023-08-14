namespace giraffe
{
    internal class Song
    {
        // static attribute is an attribute for the whole class
        // they need initial values
        public static int songCount = 0;

        public string name;
        public string artist;
        public int duration;

        public Song(string name, string artist, int duration)
        {
            this.name = name;
            this.artist = artist;
            this.duration = duration;
            songCount++;
        }

        public void Sing()
        {
            Console.WriteLine("~ {0} ~", name);
        }

        public static void Sing(string verse)
        {
            Console.WriteLine("~ {0} ~", verse);
        }

        public int GetSongCount()
        {
            // returns the global songCount
            return songCount;
        }
    }
}