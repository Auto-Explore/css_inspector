# css/css-page/layers-003-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/layers-003-print.html"
}
```

## style[0]

```css

@layer layer1, layer2;
@page b{
    margin: 3cm;
}
@layer layer1 {
    @page b {
        margin: 1cm;
    }
    @page :first {
        margin: 0;
    }
}
@layer layer2 {
    @page b {
        margin: 0;
    }
    @page a:first {
        margin: 2cm;
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
