# css/cssom/cssstyledeclaration-registered-custom-properties.html

```json
{
  "format_version": 3,
  "file": "css/cssom/cssstyledeclaration-registered-custom-properties.html"
}
```

## style[0]

```css

  @property --non-inherited-length {
    syntax: "<length>";
    inherits: false;
    initial-value: 0px;
  }
  @property --inherited-length {
    syntax: "<length>";
    inherits: true;
    initial-value: 0px;
  }
  @property --universal-with-initial {
    syntax: "*";
    inherits: false;
    initial-value: foo;
  }
  @property --universal-without-initial {
    syntax: "*";
    inherits: false;
  }
  #outer { --non-registered-outer: 1px; }
  #inner { --non-registered-inner: 2px; }
  #sibling { --universal-without-initial: bar; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
