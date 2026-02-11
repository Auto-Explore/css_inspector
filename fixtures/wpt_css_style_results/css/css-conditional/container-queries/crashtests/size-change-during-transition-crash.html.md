# css/css-conditional/container-queries/crashtests/size-change-during-transition-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/crashtests/size-change-during-transition-crash.html"
}
```

## style[0]

```css

  #outer {
    container-type: inline-size;
    width: 100px;
  }
  #inner {
    background-color: black;
    transition: background-color 60s;
  }
  #inner.target {
    background-color: white;
  }
  @container (width > 200px) {
    #inner.target  {
      background-color: lime;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
