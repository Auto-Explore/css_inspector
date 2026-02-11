# css/css-sizing/box-sizing-border-box-003-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-sizing/box-sizing-border-box-003-ref.xht"
}
```

## style[0]

```css
<![CDATA[
    .container {
      width: 300px;
      height: 400px;
      border: 2px solid black;
      position: absolute;
      left: 25px;
      top: 25px;
      background-color: red;
    }

    .box-sized {
      width: 120px;
      height: 340px;
      z-index: 1;
      float: left;
      border: 5px solid black;
      padding: 25px 10px;
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
