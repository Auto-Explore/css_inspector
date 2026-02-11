# css/css-backgrounds/background-size/reference/background-size-contain.xht

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-size/reference/background-size-contain.xht"
}
```

## style[0]

```css

      <![CDATA[
      div {
        background-color: red;
        height: 200px;
        width: 200px;
        position: relative;
        overflow: hidden;
      }
      div > img {
        height: 125px;
        width: 200px;
        position: absolute;
        left: 0;
      }
      div > img.first {
        top: 0;
      }
      div > img.second {
        top: 125px;
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
