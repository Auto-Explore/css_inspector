# css/css-conditional/container-queries/scroll-state/multiple-scroll-state-containers-comma-separated-queries.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/multiple-scroll-state-containers-comma-separated-queries.html"
}
```

## style[0]

```css

  #combined {
    container-type: inline-size scroll-state;
    overflow: scroll;
    width: 200px;
    height: 200px;
  }
  #outer {
    container: --container / scroll-state;
    overflow: scroll;
    width: 400px;
    height: 400px;
  }
  #inner {
    container-type: scroll-state;
    width: 200px;
    height: 600px;
  }
  #target {
    --match: no;
    --match-combined: no;
  }

  @container scroll-state(scrollable), --container scroll-state(scrollable) {
    #target { --match: yes; }
  }
  @container scroll-state(scrollable) {
    #target { --match: no-way; }
  }

  @container scroll-state((scrollable:right) and (scrollable:bottom)),
             (inline-size) and scroll-state((scrollable:right) and (scrollable:bottom)) {
    #target { --match-combined: yes; }
  }
  @container scroll-state((scrollable:right) and (scrollable:bottom)) {
    #target { --match-combined: no-way; }
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “container-type”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “container”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
