# css/CSS2/fonts/font-family-rule-004a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/fonts/font-family-rule-004a.xht"
}
```

## style[0]

```css
<![CDATA[
  p
  {
  font-size: 1.5em;
  font-family: serif;
  }

  span#verify1 {font-family: "CSSTest Verify";}

  div#verify {font-size: 1.5em;}

  div
  {
  font-size: 2.5em;
  line-height: 1.5;
  }

  /*
  Newest font-family syntax in CSS 2.1:
  Value:
  [[<family-name> | <generic-family>] [, [<family-name>| <generic-family>]]* ] | inherit

  Brackets ([ ]) are for grouping.

  A bar (|) separates two or more alternatives: exactly one of them must occur.

  An asterisk (*) indicates that the preceding type, word, or group occurs zero or more times.

  Section 1.4.2.1 Value
  http://www.w3.org/TR/CSS21/about.html#value-defs
  */

  div#firstTest {font-family: inherit, "CSSTest Fallback";}
  /* invalid: unquoted inherit is a reserved keyword and not a font-family name */

  div#secondTest {font-family: "CSSTest Fallback", inherit;}
  /* invalid: unquoted inherit is a reserved keyword and not a font-family name */

  div#thirdTest {font-family: inherit foo, "CSSTest Fallback";}
  /* valid in CSS 2.1: the 2 identifiers (inherit foo) form a font-family name */

  div#fourthTest {font-family: foo inherit, "CSSTest Fallback";}
  /* valid in CSS 2.1: the 2 identifiers (foo inherit) form a font-family name */

  div#fifthTest {font-family: \inherit foo, "CSSTest Fallback";}
  /* valid in CSS 2.1: the 2 identifiers (\inherit foo) form a font-family name */

  div#sixthTest {font-family: \foo inherit, "CSSTest Fallback";}
  /* valid in CSS 2.1: the 2 identifiers (\foo inherit) form a font-family name */

  div#seventhTest {font-family: "inherit" foo, "CSSTest Fallback";}
  /* invalid: it's either one string or 1+ identifiers */

  div#eighthTest {font-family: fooinherit, "CSSTest Fallback";}
  /* valid */

  div#ninthTest {font-family: inheritfoo, "CSSTest Fallback";}
  /* valid */

  div#tenthTest {font-family: --foo bar, "CSSTest Fallback";}
  /* invalid: an identifier can not start with 2 consecutive hyphens
  http://www.w3.org/TR/CSS21/syndata.html#value-def-identifier
  */

  div#eleventhTest {font-family: bar --foo, "CSSTest Fallback";}
  /* invalid: an identifier can not start with 2 consecutive hyphens
  http://www.w3.org/TR/CSS21/syndata.html#value-def-identifier
  */

  ]]>
```

```json
{
  "errors": 7,
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
      "message": "Invalid value for property “font-family”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-family”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-family”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-family”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-family”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
