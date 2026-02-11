# css/css-writing-modes/border-conflict-element-vlr-007.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/border-conflict-element-vlr-007.xht"
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
      border-style: solid solid solid solid;
      border-width: 20px 20px 20px 20px;
      padding: 0px;
    }

  td#one
    {
    border-color: green green green green;
    }

  td#two
    {
      border-color: red green green green;
    }

  td#three
    {
      border-color: red green green green;
    }

  td#four
    {
      border-color: red green green green;
    }



  td#five
    {
      border-color: green green green red;
    }

  td#six
    {
      border-color: red green green red;
    }

  td#seven
    {
      border-color: red green green red;
    }

  td#eight
    {
      border-color: red green green red;
    }



  td#nine
    {
      border-color: green green green red;
    }

  td#ten
    {
      border-color: red green green red;
    }

  td#eleven
    {
      border-color: red green green red;
    }

  td#twelve
    {
      border-color: red green green red;
    }



  td#thirteen
    {
      border-color: green green green red;
    }

  td#fourteen
    {
      border-color: red green green red;
    }

  td#fifteen
    {
      border-color: red green green red;
    }

  td#sixteen
    {
      border-color: red green green red;
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
