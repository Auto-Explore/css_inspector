# css/css-writing-modes/border-conflict-element-vrl-004.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/border-conflict-element-vrl-004.xht"
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
      padding: 0px;
    }

  td#five
    {
      border-left-color: green;
      border-left-style: solid;
      border-left-width: 100px;
    }

  td#eight
    {
      border-right-color: red;
      border-right-style: solid;
      border-right-width: 100px;
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
