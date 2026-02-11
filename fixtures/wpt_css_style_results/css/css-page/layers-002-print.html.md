# css/css-page/layers-002-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/layers-002-print.html"
}
```

## style[0]

```css

@layer layer2, layer1;
@page b{
    margin: 0;
}
@layer layer1 {
    @page {
        margin: 1cm;
    }
}
@layer layer2 {
    @page {
        margin: 0;
    }
}
div {
    width: 1cm;
    height: 1cm;
    border: 2px solid red;
}
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
