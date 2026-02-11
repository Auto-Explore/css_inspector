# css/css-conditional/container-queries/container-units-in-at-container-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-units-in-at-container-dynamic.html"
}
```

## style[0]

```css

  #outer {
     container-type: size;
     width: 40px;
     height: 40px;
   }
  #container {
    container-type: size;
    width: 16px;
    height: 16px;
  }

  @container ((width = 16px) and (width = 50cqw)) { #child { --cqw:true;  } }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
