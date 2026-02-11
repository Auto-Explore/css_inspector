# css/CSS2/margin-padding-clear/margin-collapse-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  div.ib
  {
  display: inline-block;
  left: 100px;
  position: relative;
  width: 100px;
  }

  div.ib + div.ib
  {
  position: relative;
  left: 300px;
  }

  div > div {height: 20px;}

  div.blue {background-color: blue;}

  div.orange {background-color: orange;}
  ]]>
```

```json
{
  "errors": 3,
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
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
