# css/CSS2/syntax/escapes-016.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/escapes-016.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  color: white;
  font-size: 1.25em;
  line-height: 1.2;
  }

  /*
  U+0009 is an horizontal tab
  U+000A is a line feed
  U+000B is a vertical tab
  U+000C is a form feed
  U+000D is a carriage return
  U+0020 is a blank white space
  */

  div#thirteenth
  {
  color: \9 red;
  }

  div#fourteenth
  {
  color: \9red;
  }

  div#fifteenth
  {
  color: \A red;
  }

  div#sixteenth
  {
  color: \Ared;
  }

  div#seventeenth
  {
  color: \B red;
  }

  div#eighteenth
  {
  color: \Bred;
  }

  /*
  U+0009 is an horizontal tab
  U+000A is a line feed
  U+000B is a vertical tab
  U+000C is a form feed
  U+000D is a carriage return
  U+0020 is a blank white space
  */

  div#nineteenth
  {
  color: \C red;
  }

  div#twentieth
  {
  color: \Cred;
  }

  div#twenty-first
  {
  color: \D red;
  }

  div#twenty-second
  {
  color: \Dred;
  }

  div#twenty-third
  {
  color: \20 red;
  }

  div#twenty-fourth
  {
  color: \20red;
  }

  /*
  U+0009 is an horizontal tab
  U+000A is a line feed
  U+000B is a vertical tab
  U+000C is a form feed
  U+000D is a carriage return
  U+0020 is a blank white space
  */

  ]]>
```

```json
{
  "errors": 14,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
