# css/css-writing-modes/border-conflict-element-vlr-009.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/border-conflict-element-vlr-009.xht"
}
```

## style[0]

```css
<![CDATA[
  table
    {
      border-collapse: collapse;
      direction: ltr;
      writing-mode: vertical-lr;
    }

  td
    {
      border-style: solid none solid none;
      border-width: 20px 0px 20px 0px;
      padding: 0px;
      width: 25px; /* height of each logical row */
    }

  td#one
    {
      border-color: green red green red;
    }

  td#two
    {
      border-color: red red green red;
    }

  td#three
    {
      border-color: red red green red;
    }

  td#four
    {
      border-color: red red green red;
    }



  td#five
    {
      border-color: green red green red;
    }

  td#six
    {
      border-color: red red green red;
    }

  td#seven
    {
      border-color: red red green red;
    }

  td#eight
    {
      border-color: red red green red;
    }



  td#nine
    {
      border-color: green red green red;
    }

  td#ten
    {
      border-color: red red green red;
    }

  td#eleven
    {
      border-color: red red green red;
    }

  td#twelve
    {
      border-color: red red green red;
    }



  td#thirteen
    {
      border-color: green red green red;
    }

  td#fourteen
    {
      border-color: red red green red;
    }

  td#fifteen
    {
      border-color: red red green red;
    }

  td#sixteen
    {
      border-color: red red green red;
    }
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
