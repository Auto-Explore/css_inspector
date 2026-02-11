# css/css-sizing/box-sizing-content-box-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-sizing/box-sizing-content-box-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
    .container {
      width: 300px;
      height: 110px;
      border: 2px solid black;
      position: absolute;
      left: 25px;
      top: 25px;
      background-color: red;
    }

    .box-sized {
      width: 150px;
      height: 110px;
      z-index: 1;
      float: left;
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
