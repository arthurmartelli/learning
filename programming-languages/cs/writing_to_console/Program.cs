using System;

namespace writing_to_console
{
    class Program
    {
        static void Main(string[] args)
        {
            string name = get_from_user("Ingrese su nombre: ");

            int birth_year = get_int_from_user("Ingrese su año de nacimiento: ");
            int age = calculate_age(birth_year);

            string message = string.Format(format: "{0}, su edad es {1} años", name, age.ToString());
            Console.WriteLine(message);
        }

        static string get_from_user(string message)
        {
            Console.Write(message);
            return Console.ReadLine();
        }

        static int calculate_age(int birth_year)
        {
            return DateTime.Now.Year - birth_year;
        }

        static int get_int_from_user(string message)
        {
            int result;
            string value = get_from_user(message);

            while (!int.TryParse(value, out result))
            {
                value = get_from_user(message);
            }

            return result;
        }
    }
}