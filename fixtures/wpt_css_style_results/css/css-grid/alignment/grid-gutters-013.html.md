# css/css-grid/alignment/grid-gutters-013.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-gutters-013.html"
}
```

## style[0]

```css

    #grid {
        display: grid;
        margin-top: -50px;
        margin-left: -50px;
        width: 500px;
        height: 500px;
        grid-gap: 100px;
        grid-template-rows: repeat(auto-fit, 200px);
        grid-template-columns: repeat(auto-fit, 200px);
        align-items: center;
        justify-items: center;
        background: linear-gradient(red, red) no-repeat 50px 50px / 100px 100px;
    }

    #grid > div {
        background-color: green;
        width: 50%;
        height: 50%;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
