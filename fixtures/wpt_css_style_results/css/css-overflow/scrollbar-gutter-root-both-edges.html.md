# css/css-overflow/scrollbar-gutter-root-both-edges.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scrollbar-gutter-root-both-edges.html"
}
```

## style[0]

```css

    body,html {
        background-color: blue;
        margin: 10px;
        padding: 10px;
        border: 5px solid black;
    }

    body{
        border: 5px solid green;
    }


:root {
  scrollbar-gutter: stable both-edges;
}
p {
  background-color: purple;
  color: white;
}
div {
    position:absolute;
    left:-20px;
    top:150px;
    background-color: green;
    width: 100px;
    height: 100px;
    transform: translateZ(0);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
