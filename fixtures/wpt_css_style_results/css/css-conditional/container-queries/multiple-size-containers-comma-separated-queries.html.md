# css/css-conditional/container-queries/multiple-size-containers-comma-separated-queries.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/multiple-size-containers-comma-separated-queries.html"
}
```

## style[0]

```css

  #outer {
    container: --container / inline-size;
    width: 500px;
  }
  #inner {
    container-type: inline-size;
    width: 300px;
  }
  #target { --match: no; }
  @container (width > 400px), --container (width > 400px) {
    #target { --match: yes; }
  }
  @container (width > 400px) {
    #target { --match: no-way; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
