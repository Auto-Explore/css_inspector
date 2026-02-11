# css/css-transforms/css-transform-inherit-scale.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/css-transform-inherit-scale.html"
}
```

## style[0]

```css

      .test {
          position: relative;
      }
      .red {
          position: absolute;
          width: 200px;
          height: 200px;
          background-color: red;
      }
      .parent {
          background: yellow;
          width: 50px;
          height: 50px;
          position: absolute;
          top: 75px;
          left: 75px;
          transform: scale(2);

      }
      .child {
          position: absolute;
          transform: inherit;
          width: 50px;
          height: 50px;
          background-color: green;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
