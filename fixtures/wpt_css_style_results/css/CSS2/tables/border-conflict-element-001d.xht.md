# css/CSS2/tables/border-conflict-element-001d.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-conflict-element-001d.xht"
}
```

## style[0]

```css
<![CDATA[
  table
  {
  border-collapse: collapse;
  color: white;
  font: 1.25em/1 Ahem;
  margin: auto auto 2em 2em;
  }

  td {padding: 0px;}

  table#test td
  {
  border-style: solid;
  border-width: 1em;
  }

  td#one {border-color: blue blue blue blue;}
  td#two {border-color: yellow yellow yellow red;}
  td#three {border-color: green green green red;}
  td#four {border-color: orange orange orange red;}

  td#five {border-color: red yellow yellow yellow;}
  td#six {border-color: red green green red;}
  td#seven {border-color: red orange orange red;}
  td#eight {border-color: red blue blue red;}

  td#nine {border-color: red green green green;}
  td#ten {border-color: red orange orange red;}
  td#eleven {border-color: red blue blue red;}
  td#twelve {border-color: red yellow yellow red;}

  td#thirteen {border-color: red orange orange orange;}
  td#fourteen {border-color: red blue blue red;}
  td#fifteen {border-color: red yellow yellow red;}
  td#sixteen {border-color: red green green red;}

  img
  {
  height: 1em;
  vertical-align: bottom;
  /*
  With 'vertical-align: bottom', swatch-[color] images "sit"
  at the bottom of the cell's line box and not on its baseline
  */
  width: 1em;
  }
  ]]>
```

```json
{
  "errors": 6,
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
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
