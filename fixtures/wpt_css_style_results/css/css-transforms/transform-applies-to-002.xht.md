# css/css-transforms/transform-applies-to-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-applies-to-002.xht"
}
```

## style[0]

```css
<![CDATA[
    div {
      position: relative;
      color: green;
    }
    div p {
      width: 10em;
      font-family: Ahem;
      position: absolute;
      top: 0;
      left: 0;
      line-height: 2;
    }
    #red span {
      color: red;
    }
    #test span {
      transform: rotate(90deg);
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
