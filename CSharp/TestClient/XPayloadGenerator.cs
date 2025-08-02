using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;

using UsingMiddleware;

namespace TestClient
{
    public class XPayloadGenerator
    {
        private static readonly Random _random = new Random(42); 
        private static int _personIdCounter = 1;

        public static XPayload Create3KBPayload()
        {
            var payload = new XPayload
            {
                ByteField = (byte)_random.Next(0, 255),
                SByteField = (sbyte)_random.Next(-128, 127),
                ShortField = (short)_random.Next(-32768, 32767),
                UShortField = (ushort)_random.Next(0, 65535),
                IntField = _random.Next(),
                UIntField = (uint)_random.Next(),
                LongField = _random.NextInt64(),
                ULongField = (ulong)_random.NextInt64() & 0x7FFFFFFFFFFFFFFF,

                FloatField = (float)(_random.NextDouble() * 1000),
                DoubleField = _random.NextDouble() * 10000,

                DecimalField = (decimal)(_random.NextDouble() * 100000),

                BoolField = _random.Next(2) == 1,

                CharField = (char)_random.Next(65, 90), 
                StringField = GenerateRandomString(200), 

                GuidField = Guid.NewGuid(),

                DateTimeField = DateTime.UtcNow.AddDays(_random.Next(-365, 365)),
                TimeSpanField = TimeSpan.FromMinutes(_random.Next(0, 1440)), // 0-24 horas

                Persons = GeneratePersonList(8) 
            };

            var options = new JsonSerializerOptions
            {
                WriteIndented = false, 
                PropertyNamingPolicy = JsonNamingPolicy.CamelCase
            };

            return payload;
        }

        private static List<XPersonData> GeneratePersonList(int count)
        {
            var persons = new List<XPersonData>();

            for (int i = 0; i < count; i++)
            {
                persons.Add(new XPersonData
                {
                    Id = _personIdCounter++,
                    PersonGuid = Guid.NewGuid(),
                    FirstName = GenerateFirstName(),
                    LastName = GenerateLastName(),
                    Age = _random.Next(18, 80),
                    BirthDate = DateTime.UtcNow.AddYears(-_random.Next(18, 80)).AddDays(_random.Next(0, 365)),
                    Email = GenerateEmail(),
                    PhoneNumber = GeneratePhoneNumber(),
                    Address = GenerateAddress(),
                    IsActive = _random.Next(100) < 95 // 95% de ativos
                });
            }

            return persons;
        }

        private static XAddressInfo GenerateAddress()
        {
            return new XAddressInfo
            {
                Street = GenerateStreetName(),
                Number = _random.Next(1, 5000).ToString(),
                Complement = _random.Next(3) == 0 ? GenerateRandomString(10) : null,
                Neighborhood = GenerateNeighborhood(),
                City = GenerateCity(),
                State = GenerateState(),
                PostalCode = GeneratePostalCode(),
                Country = "Brasil",
                Latitude = _random.NextDouble() * 180 - 90,
                Longitude = _random.NextDouble() * 360 - 180
            };
        }

        private static string GenerateRandomString(int length)
        {
            const string chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 áéíóúàèìòùâêîôûãõç";
            var stringChars = new char[length];

            for (int i = 0; i < length; i++)
            {
                stringChars[i] = chars[_random.Next(chars.Length)];
            }

            return new string(stringChars);
        }

        private static string GenerateFirstName()
        {
            var firstNames = new[]
            {
            "Maria", "João", "Ana", "Pedro", "Carlos", "Fernanda", "Roberto", "Juliana",
            "Marcos", "Patrícia", "Alexandre", "Camila", "Ricardo", "Beatriz", "Gustavo",
            "Larissa", "Felipe", "Amanda", "Daniel", "Carolina", "Thiago", "Renata",
            "Vinícius", "Tatiane", "Eduardo", "Vanessa", "Anderson", "Priscila",
            "Leonardo", "Débora", "Marcelo", "Simone", "Rafael", "Cristina"
        };

            return firstNames[_random.Next(firstNames.Length)];
        }

        private static string GenerateLastName()
        {
            var lastNames = new[]
            {
            "Silva", "Santos", "Oliveira", "Souza", "Pereira", "Costa", "Carvalho",
            "Almeida", "Ferreira", "Ribeiro", "Alves", "Monteiro", "Barbosa",
            "Lima", "Mendes", "Freitas", "Barros", "Pinto", "Vieira", "Araújo",
            "Rocha", "Martins", "Rodrigues", "Nascimento", "Campos", "Machado",
            "Cardoso", "Teixeira", "Medeiros", "Nunes", "Moraes", "Machado"
        };

            return lastNames[_random.Next(lastNames.Length)];
        }

        private static string GenerateEmail()
        {
            var domains = new[] { "gmail.com", "hotmail.com", "yahoo.com", "outlook.com", "empresa.com.br" };
            var firstName = GenerateFirstName().ToLower().Replace(" ", ".");
            var lastName = GenerateLastName().ToLower().Replace(" ", ".");
            var domain = domains[_random.Next(domains.Length)];

            return $"{firstName}.{lastName}{_random.Next(1, 999)}@{domain}";
        }

        private static string GeneratePhoneNumber()
        {
            return $"({_random.Next(11, 99)}) 9{_random.Next(1000, 9999)}-{_random.Next(1000, 9999)}";
        }

        private static string GenerateStreetName()
        {
            var streets = new[]
            {
            "Rua das Flores", "Avenida Brasil", "Rua dos Andradas", "Avenida Paulista",
            "Rua Oscar Freire", "Rua Augusta", "Avenida Rio Branco", "Rua XV de Novembro",
            "Avenida Atlântica", "Rua da Consolação", "Avenida Brigadeiro Faria Lima",
            "Rua Bela Vista", "Avenida Copacabana", "Rua Direita", "Avenida Europa"
        };

            return streets[_random.Next(streets.Length)];
        }

        private static string GenerateNeighborhood()
        {
            var neighborhoods = new[]
            {
            "Centro", "Jardins", "Moema", "Vila Madalena", "Pinheiros", "Ipanema",
            "Copacabana", "Leblon", "Botafogo", "Flamengo", "Lapa", "Santa Teresa",
            "Barra da Tijuca", "Recreio", "Tijuca", "Méier", "Copacabana", "Botafogo"
        };

            return neighborhoods[_random.Next(neighborhoods.Length)];
        }

        private static string GenerateCity()
        {
            var cities = new[]
            {
            "São Paulo", "Rio de Janeiro", "Belo Horizonte", "Porto Alegre",
            "Curitiba", "Salvador", "Fortaleza", "Brasília", "Recife", "Manaus"
        };

            return cities[_random.Next(cities.Length)];
        }

        private static string GenerateState()
        {
            var states = new[]
            {
            "SP", "RJ", "MG", "RS", "PR", "BA", "CE", "DF", "PE", "AM"
        };

            return states[_random.Next(states.Length)];
        }

        private static string GeneratePostalCode()
        {
            return $"{_random.Next(10000, 99999)}-{_random.Next(100, 999)}";
        }

        public static List<XPayload> CreateMultiplePayloads(int count)
        {
            var payloads = new List<XPayload>();
            for (int i = 0; i < count; i++)
            {
                payloads.Add(Create3KBPayload());
            }
            return payloads;
        }
    }
}
