using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace giraffe
{
    class Program
    {
        static void Main(string[] args)
        {
            // tutorial done by following https://youtu.be/GhQdlIFylQ8

            Inheritance();
        }

        static void Inheritance()
        {
            Chef chef = new Chef();
            chef.MakeChicken();

            ItalianChef italianChef = new ItalianChef();
            italianChef.MakeChicken();
            italianChef.MakeSpaghetti();
            italianChef.MakeSpecialDish();
        }

        static void StaticMethods()
        {
            // static method are methods that don't
            // need a class instance to be called
            Song.Sing("Hello!");
        }

        static void StaticAttributes()
        {
            var song1 = new Song("Holiday", "Green Day", 200);
            var song2 = new Song("Kashmin", "Led Zeppelin", 150);

            song1.Sing();
            Console.WriteLine(Song.songCount);
        }

        static void GetterSetter()
        {
            var movie1 = new Movie("The Avengers", "Joss Whedon", "PG-13");
            var movie2 = new Movie("Shrek", "Adam Adamson", "PG");
        }

        static void ClassMethods()
        {
            Student student1 = new Student("Jim", "Business", 2.8);
            Student student2 = new Student("Pam", "Art", 3.6);

            Console.WriteLine(student1.HasHonors());
            Console.WriteLine(student2.HasHonors());
        }

        static void Classes()
        {
            // create new, empty book
            Book myBook = new Book("Harry Potter", "JK Rowling", 400);

            // Book with prefilled data
            Book myNewBook = new Book("Lord of the Ring", "Tolkein", 700);

            Console.WriteLine(myBook.title);
        }

        static void ExceptionHandling()
        {
            try
            {
                Console.Write("Enter a number: ");
                double num1 = Convert.ToDouble(Console.ReadLine());
                Console.Write("Enter another number: ");
                double num2 = Convert.ToDouble(Console.ReadLine());

                Console.WriteLine(num1 / num2);
            }
            catch (DivideByZeroException e)
            {
                // only executes when error is met
                Console.WriteLine(e.Message);
            }
            catch (Exception e)
            {
                Console.WriteLine(e.Message);
            }
            finally
            {
                // always executes
                Console.WriteLine("I hope it worked!");
            }
        }

        static void Comments()
        {
            // Comment text for a good explanation
            Console.WriteLine("Comments are fun!");

            /*
            Multiline
                Comments!
            */
        }

        static void Array2D()
        {
            int[,] numberGrid = {
                { 1, 2 },
                { 3, 4 },
                { 5, 6 },
            };

            Console.WriteLine(numberGrid[1, 0]);
        }

        static int Exponent(int baseNum, int powNum)
        {
            int result = 1;

            for (int i = 0; i < powNum; i++)
            {
                result *= baseNum;
            }

            return result;
        }

        static void ForLoop()
        {
            int[] numbers = { 1, 2, 3, 4 };

            for (int i = 0; i < numbers.Length; i++)
            {
                Console.WriteLine(numbers[i]);
            }
        }

        static void GuessingGame()
        {
            string secretWord = "giraffe";
            string? guess;
            int lives = 2;

            do
            {
                Console.Write("Enter the guess: ");
                guess = Console.ReadLine();
                lives--;
            }
            while ((secretWord != guess) && (lives > 0));

            if (lives == 0)
            {
                Console.WriteLine("You Lost!");
                return;
            }

            Console.WriteLine("You Win!");
        }

        static void WhileLoop()
        {
            int index1 = 4;

            do
            {
                Console.WriteLine("At index {0}", index1);
                index1++;
            } while (index1 < 3);

            int index2 = 1;

            while (index2 < 10)
            {
                Console.WriteLine("At index {0}", index2);
                index2++;
            }
        }

        static string SwitchStatement(int number)
        {
            string dayName;

            switch (number)
            {
                case 0:
                    dayName = "Sunday";
                    break;

                case 1:
                    dayName = "Monday";
                    break;

                case 2:
                    dayName = "Tuesday";
                    break;

                case 3:
                    dayName = "Wednesday";
                    break;

                case 4:
                    dayName = "Thursday";
                    break;

                case 5:
                    dayName = "Friday";
                    break;

                case 6:
                    dayName = "Saturday";
                    break;

                default:
                    dayName = "Invalid Day Number";
                    break;
            }
            return dayName;
        }

        static void BetterCalculator()
        {
            Console.Write("Enter your first number: ");
            double num1 = Convert.ToDouble(Console.ReadLine());

            Console.Write("Enter your operator: ");
            string? op = Console.ReadLine();

            Console.Write("Enter your second number: ");
            double num2 = Convert.ToDouble(Console.ReadLine());

            if (op == "+")
            {
                Console.WriteLine(num1 + num2);
                return;
            }

            if (op == "-")
            {
                Console.WriteLine(num1 - num2);
                return;
            }

            if (op == "*")
            {
                Console.WriteLine(num1 * num2);
                return;
            }

            if (op == "/")
            {
                Console.WriteLine(num1 / num2);
                return;
            }

            Console.WriteLine("Invalid Operator");
        }

        static int MoreIfStatement(int[] numbers)
        {
            int result = int.MinValue;

            foreach (int number in numbers)
            {
                if (number > result)
                {
                    result = number;
                }
            }

            return result;
        }

        static void IfStatement()
        {
            bool isMale = false;
            bool isTall = false;

            if (!isMale && !isTall)
            {
                Console.WriteLine("You are not male nor tall");
                return;
            }

            if (!isMale)
            {
                Console.WriteLine("You are not male but you are tall");
                return;
            }

            if (!isTall)
            {

                Console.WriteLine("You are not tall but you are male");
                return;
            }

            Console.WriteLine("You are a tall male");
        }

        static double ReturnStatement(double number)
        {
            return Math.Pow(number, 3);
        }

        static void Methods(string name)
        {
            Console.WriteLine("Hello User {0}!", name);
        }

        static void Arrays()
        {
            int[] numbers = { 1, 2, 6, 19, 55 };
            string[] friends = new string[5];

            friends[0] = "Jim";
            friends[1] = "Pam";
            friends[2] = "Grace";
            friends[3] = "John";
            friends[4] = "Mike";

            for (int i = 0; i < numbers.Length; i++)
            {
                Console.WriteLine(numbers[i]);
            }

            foreach (string friend in friends)
            {
                Console.WriteLine(friend);
            }
        }

        static void MadLib()
        {
            string? color, noun, celebrity;

            Console.Write("Enter a color: ");
            color = Console.ReadLine();

            Console.Write("Enter a noun: ");
            noun = Console.ReadLine();

            Console.Write("Enter a celebrity: ");
            celebrity = Console.ReadLine();

            Console.WriteLine("Roses are {0}", color);
            Console.WriteLine("{0} are blue", noun);
            Console.WriteLine("I Love {0}", celebrity);
        }

        static void Calculator()
        {
            // int num = (int)Convert.ToInt32("76");
            Console.Write("Enter a number: ");
            double num1 = Convert.ToDouble(Console.ReadLine());
            Console.Write("Enter another number: ");
            double num2 = Convert.ToDouble(Console.ReadLine());

            Console.WriteLine(num1 + num2);
        }

        static void UserInput()
        {
            Console.Write("Input your name: ");
            string? name = Console.ReadLine();
            Console.Write("Input your name: ");
            string? age = Console.ReadLine();
            Console.WriteLine("Hello {0}, you are {1} years old", name, age);
        }

        static void WorkingWithNumbers()
        {
            Console.WriteLine((decimal)5 / 2); // casting the result of int / int into decimal
            decimal num = (decimal)Math.PI;
            Console.WriteLine(num);
        }

        static void WorkingWithStrings()
        {
            string phrase = "   Giraffe\n\"Program\"" + " is cool";
            phrase = phrase.Trim().ToUpper();
            Console.WriteLine(phrase);
        }

        static void DataTypes()
        {
            string phrase = "Giraffe Program";
            char grade = 'A';
            int age = 30;
            // float, double, decimal for decimals
            double gpa = 3.3;
            bool isMale = true;
            Console.WriteLine(phrase, grade, age, gpa, isMale);
        }

        static void Variables()
        {
            string name = "George";
            int age = 54;

            Console.WriteLine("There once was a man named {0}", name);
            Console.WriteLine("He was {0} years old", age);
            Console.WriteLine("He really liked the name {0}", name);
            Console.WriteLine("But didn't like being {0}", age);
        }
    }
}