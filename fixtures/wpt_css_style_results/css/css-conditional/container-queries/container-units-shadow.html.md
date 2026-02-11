# css/css-conditional/container-queries/container-units-shadow.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-units-shadow.html"
}
```

## style[0]

```css

  #outer {
    container-type: size;
    width: 200px;
    height: 200px;
  }
  #direct {
    container-type: inline-size;
    width: 50cqw;
    height: 50cqh;
  }
  #nondirect {
    width: 10cqw;
    height: 10cqh;
    background: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

        #inner {
          container-type: size;
          width: 30px;
          height: 30px;
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
