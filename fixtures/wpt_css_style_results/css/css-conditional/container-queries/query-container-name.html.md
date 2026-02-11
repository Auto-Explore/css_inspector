# css/css-conditional/container-queries/query-container-name.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/query-container-name.html"
}
```

## style[0]

```css

  #inner { container-name: --foo }
  #outer { container-name: --bar }
  #target {
    --match-foo: no;
    --match-bar: no;
    --match-baz: no;
  }
  @container --foo { #target { --match-foo: yes; } }
  @container --bar { #target { --match-bar: yes; } }
  @container --baz { #target { --match-baz: yes; } }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
