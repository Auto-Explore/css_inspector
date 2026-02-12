# css/css-writing-modes/border-conflict-element-vlr-005.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/border-conflict-element-vlr-005.xht"
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
      padding: 0px;
    }

  td#five
    {
      border-right-color: green;
      border-right-style: solid;
      border-right-width: 100px;
    }

  td#eight
    {
      border-left-color: red;
      border-left-style: solid;
      border-left-width: 100px;
      height: 100px;
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
