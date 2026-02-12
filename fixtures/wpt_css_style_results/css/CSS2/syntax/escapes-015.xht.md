# css/CSS2/syntax/escapes-015.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/escapes-015.xht"
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

  div#first
  {
  color: red \9;
  }

  div#second
  {
  color: red\9;
  }

  div#third
  {
  color: red \A;
  }

  div#fourth
  {
  color: red\A;
  }

  div#fifth
  {
  color: red \B;
  }

  div#sixth
  {
  color: red\B;
  }

  /*
  U+0009 is an horizontal tab
  U+000A is a line feed
  U+000B is a vertical tab
  U+000C is a form feed
  U+000D is a carriage return
  U+0020 is a blank white space
  */

  div#seventh
  {
  color: red \C;
  }

  div#eighth
  {
  color: red\C;
  }

  div#ninth
  {
  color: red \D;
  }

  div#tenth
  {
  color: red\D;
  }

  div#eleventh
  {
  color: red \20;
  }

  div#twelfth
  {
  color: red\20;
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
  "errors": 12,
  "messages": [
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
