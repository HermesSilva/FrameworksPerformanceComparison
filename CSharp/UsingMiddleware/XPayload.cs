using System;
using System.Collections.Generic;

namespace UsingMiddleware
{
    public class XPayload
    {
        public byte ByteField
        {
            get; set;
        }
        public sbyte SByteField
        {
            get; set;
        }
        public short ShortField
        {
            get; set;
        }
        public ushort UShortField
        {
            get; set;
        }
        public int IntField
        {
            get; set;
        }
        public uint UIntField
        {
            get; set;
        }
        public long LongField
        {
            get; set;
        }
        public ulong ULongField
        {
            get; set;
        }

        public float FloatField
        {
            get; set;
        }
        public double DoubleField
        {
            get; set;
        }
        
        public decimal DecimalField
        {
            get; set;
        }
        
        public bool BoolField
        {
            get; set;
        }
        
        public char CharField
        {
            get; set;
        }
        public string StringField
        {
            get; set;
        }
        
        public Guid GuidField
        {
            get; set;
        }
        public DateTime DateTimeField
        {
            get; set;
        }


        public TimeSpan TimeSpanField
        {
            get; set;
        }

        public List<XPersonData> Persons { get; set; } = new List<XPersonData>();
    }
    public class XPersonData
    {
        public int Id
        {
            get; set;
        }
        public Guid PersonGuid
        {
            get; set;
        }
        public string FirstName
        {
            get; set;
        }
        public string LastName
        {
            get; set;
        }

        public int Age
        {
            get; set;
        }
        public DateTime BirthDate
        {
            get; set;
        }
        public string Email
        {
            get; set;
        }

        public string PhoneNumber
        {
            get; set;
        }

        public XAddressInfo Address
        {
            get; set;
        }

        public bool IsActive
        {
            get; set;
        }
    }

    public class XAddressInfo
    {
        public string Street
        {
            get; set;
        }
        public string Number
        {
            get; set;
        }
        public string Complement
        {
            get; set;
        }
        public string Neighborhood
        {
            get; set;
        }
        public string City
        {
            get; set;
        }
        public string State
        {
            get; set;
        }
        public string PostalCode
        {
            get; set;
        }
        public string Country
        {
            get; set;
        }
        public double Latitude
        {
            get; set;
        }
        public double Longitude
        {
            get; set;
        }
    }
}
