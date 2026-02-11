# css/css-sizing/box-sizing-border-box-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-sizing/box-sizing-border-box-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
    .container {
      width: 300px;
      border: 2px solid black;
      position: absolute;
      left: 25px;
      top: 25px;
      background-color: red;
    }

    .box-sized {
      width: 140px;
      z-index: 1;
      float: left;
      border: 5px solid black;
    }

    #one {
      background-color: green;
    }

    #two {
      background-color: blue;
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
