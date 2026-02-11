# css/CSS2/fonts/font-052.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/fonts/font-052.xht"
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
  font shorthand syntax in CSS 2.1:

  Value:  	[ [ <'font-style'> || <'font-variant'> || <'font-weight'> ]? <'font-size'> [ / <'line-height'> ]? <'font-family'> ] | caption | icon | menu | message-box | small-caption | status-bar | inherit

  Brackets ([ ]) are for grouping.

  A double bar (||) separates two or more options: one or more of them must occur, in any order.

  A question mark (?) indicates that the preceding type, word, or group is optional.

  A bar (|) separates two or more alternatives: exactly one of them must occur.

  Section 1.4.2.1 Value
  http://www.w3.org/TR/CSS21/about.html#value-defs
  */

  div#firstTest {font: 40px \inherit foo, "CSSTest Fallback";}
  /* valid: there can be a font whose name is "inherit foo" */

  div#secondTest {font: 40px foo \inherit, "CSSTest Fallback";}
  /* valid: there can be a font whose name is "foo inherit" */

  div#thirdTest {font: caption;}
  /* valid: caption is the reserved keyword for the system font */

  div#fourthTest {font: medium caption, "CSSTest Fallback";}
  /* valid: there can be a font whose name is "caption";
  "medium" is the font-size */

  div#fifthTest {font: caption foo, "CSSTest Fallback";}
  /* invalid: there could be a font whose name is "caption foo"
  but then the font shorthand declaration would have no font-size */

  div#sixthTest {font: caption, foo, "CSSTest Fallback";}
  /* invalid */

  div#seventhTest {font: medium caption, inherit, "CSSTest Fallback";}
  /* invalid */

  /*
  font shorthand syntax in CSS 2.1:

  Value:  	[ [ <'font-style'> || <'font-variant'> || <'font-weight'> ]? <'font-size'> [ / <'line-height'> ]? <'font-family'> ] | caption | icon | menu | message-box | small-caption | status-bar | inherit

  Brackets ([ ]) are for grouping.

  A double bar (||) separates two or more options: one or more of them must occur, in any order.

  A question mark (?) indicates that the preceding type, word, or group is optional.

  A bar (|) separates two or more alternatives: exactly one of them must occur.

  Section 1.4.2.1 Value
  http://www.w3.org/TR/CSS21/about.html#value-defs
  */

  ]]>
```

```json
{
  "errors": 5,
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
      "message": "Invalid value for property “font”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
