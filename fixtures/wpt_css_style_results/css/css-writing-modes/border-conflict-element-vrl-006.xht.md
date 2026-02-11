# css/css-writing-modes/border-conflict-element-vrl-006.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/border-conflict-element-vrl-006.xht"
}
```

## style[0]

```css
<![CDATA[
  table
    {
      border-collapse: collapse;
      direction: ltr;
      writing-mode: vertical-rl;
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
      border-color: green red green green;
    }

  td#six
    {
      border-color: red red green green;
    }

  td#seven
    {
      border-color: red red green green;
    }

  td#eight
    {
      border-color: red red green green;
    }



  td#nine
    {
      border-color: green red green green;
    }

  td#ten
    {
      border-color: red red green green;
    }

  td#eleven
    {
      border-color: red red green green;
    }

  td#twelve
    {
      border-color: red red green green;
    }



  td#thirteen
    {
      border-color: green red green green;
    }

  td#fourteen
    {
      border-color: red red green green;
    }

  td#fifteen
    {
      border-color: red red green green;
    }

  td#sixteen
    {
      border-color: red red green green;
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
