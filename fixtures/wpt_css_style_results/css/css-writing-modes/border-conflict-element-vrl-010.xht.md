# css/css-writing-modes/border-conflict-element-vrl-010.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/border-conflict-element-vrl-010.xht"
}
```

## style[0]

```css
<![CDATA[
  table
    {
      border-collapse: collapse;
      direction: rtl;
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
      border-color: green green red green;
    }

  td#three
    {
      border-color: green green red green;
    }

  td#four
    {
      border-color: green green red green;
    }



  td#five
    {
      border-color: green red green green;
    }

  td#six
    {
      border-color: green red red green;
    }

  td#seven
    {
      border-color: green red red green;
    }

  td#eight
    {
      border-color: green red red green;
    }



  td#nine
    {
      border-color: green red green green;
    }

  td#ten
    {
      border-color: green red red green;
    }

  td#eleven
    {
      border-color: green red red green;
    }

  td#twelve
    {
      border-color: green red red green;
    }



  td#thirteen
    {
      border-color: green red green green;
    }

  td#fourteen
    {
      border-color: green red red green;
    }

  td#fifteen
    {
      border-color: green red red green;
    }

  td#sixteen
    {
      border-color: green red red green;
    }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
