# css/css-conditional/container-queries/crashtests/top-layer-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/crashtests/top-layer-crash.html"
}
```

## style[0]

```css

                              .wrapper {
                                container: example / inline-size;
                                position: fixed;
                                width: 100%;
                              }
                              .inner {
                                display: none;
                              }
                              @container (min-width: 600px) {
                                .inner {
                                  display: block;
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
