# css/css-position/containing-block-change-button.html

```json
{
  "format_version": 3,
  "file": "css/css-position/containing-block-change-button.html"
}
```

## style[0]

```css

  #button {
    width: 400px;
    height: 400px;
    margin: 50px;
    background-color: purple;
    border: 0;
    padding: 10px;
  }
  #button > div {
    position: absolute;
    width: 100px;
    height: 100px;
    background-color: green;
    /* Vertically centered to match in-flow button layout of the reference */
    top: 150px;
    left: 0;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
