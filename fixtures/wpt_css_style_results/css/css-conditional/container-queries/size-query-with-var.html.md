# css/css-conditional/container-queries/size-query-with-var.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/size-query-with-var.html"
}
```

## style[0]

```css

  @property --registered {
    syntax: "<length>";
    inherits: false;
    initial-value: 0;
  }
  @property --registered-keyword {
    syntax: "none|fail";
    inherits: false;
    initial-value: none;
  }
  @property --registered-number {
    syntax: "<number>";
    inherits: false;
    initial-value: 0;
  }
  #container {
    width: 400px;
    container-type: inline-size;
    --unregistered: 200px;
    --unregistered-keyword: fail;
    --unregistered-number: 200;
    --registered: 200px;
    --registered-keyword: fail;
    --registered-number: 0;
  }
  #target {
    --match-unknown: no;
    --match-unknown-fallback: no;
    --match-unregistered: no;
    --match-unregistered-keyword: no;
    --match-unregistered-number: no;
    --match-registered: no;
    --match-registered-keyword: no;
    --match-registered-number: no;
  }
  @container (width > var(--unknown)) {
    #target { --match-unknown: yes; }
  }
  @container (width > var(--unknown, 100px)) {
    #target { --match-unknown-fallback: yes; }
  }
  @container (width > var(--unregistered)) {
    #target { --match-unregistered: yes; }
  }
  @container (width > var(--unregistered-keyword)) {
    #target { --match-unregistered-keyword: yes; }
  }
  @container (width > var(--unregistered-number)) {
    #target { --match-unregistered-number: yes; }
  }
  @container (width > var(--registered)) {
    #target { --match-registered: yes; }
  }
  @container (width > var(--registered-keyword)) {
    #target { --match-registered-keyword: yes; }
  }
  @container (width > var(--registered-number)) {
    #target { --match-registered-number: yes; }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
