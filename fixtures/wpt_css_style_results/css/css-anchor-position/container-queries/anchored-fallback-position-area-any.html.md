# css/css-anchor-position/container-queries/anchored-fallback-position-area-any.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/container-queries/anchored-fallback-position-area-any.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --a;
    width: 100px;
    height: 100px;
  }
  #anchored {
    position: absolute;
    position-anchor: --a;
    position-area: top;
    position-try-fallbacks: span-bottom;
    width: 100px;
    /* Too tall to fit over the anchor to trigger fallback */
    height: 100px;
    container-type: anchored;
  }
  #target {
    @container anchored(fallback: any) { --any: yes; }
    @container anchored(fallback: any span-bottom) { --any-span-bottom: yes; }
    @container anchored(fallback: span-end any) { --span-end-any: yes; }
    @container anchored(fallback: any span-all) { --any-span-all: yes; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
