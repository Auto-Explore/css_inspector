# css/css-writing-modes/border-conflict-element-vlr-011.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/border-conflict-element-vlr-011.xht"
}
```

## style[0]

```css
<![CDATA[
  table
    {
      border-collapse: collapse;
      direction: rtl;
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
      border-color: green green green red;
    }

  td#six
    {
      border-color: green green red red;
    }

  td#seven
    {
      border-color: green green red red;
    }

  td#eight
    {
      border-color: green green red red;
    }



  td#nine
    {
      border-color: green green green red;
    }

  td#ten
    {
      border-color: green green red red;
    }

  td#eleven
    {
      border-color: green green red red;
    }

  td#twelve
    {
      border-color: green green red red;
    }



  td#thirteen
    {
      border-color: green green green red;
    }

  td#fourteen
    {
      border-color: green green red red;
    }

  td#fifteen
    {
      border-color: green green red red;
    }

  td#sixteen
    {
      border-color: green green red red;
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
