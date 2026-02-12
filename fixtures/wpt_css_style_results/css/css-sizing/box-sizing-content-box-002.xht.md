# css/css-sizing/box-sizing-content-box-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-sizing/box-sizing-content-box-002.xht"
}
```

## style[0]

```css
<![CDATA[
    .container {
      width: 300px;
      height: 100px;
      border: 2px solid black;
      position: absolute;
      left: 25px;
      top: 25px;
      background-color: red;
    }

    .box-sized {
      box-sizing: content-box;
      width: calc(50% - 10px);
      height: calc(100% - 10px);
      z-index: 1;
      float: left;
      padding: 5px 5px;
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
