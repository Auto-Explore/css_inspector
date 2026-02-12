# css/css-sizing/box-sizing-content-box-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-sizing/box-sizing-content-box-003.xht"
}
```

## style[0]

```css
<![CDATA[
    .container {
      min-width: 500px;
      max-width: 700px;
      height: 90px;
      border: 2px solid black;
      position: absolute;
      left: 25px;
      top: 25px;
      background-color: red;
    }

    .box-sized {
      box-sizing: content-box;
      width: 50%;
      height: 100%;
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
