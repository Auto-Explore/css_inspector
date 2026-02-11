# css/css-conditional/container-queries/at-container-style-serialization.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/at-container-style-serialization.html"
}
```

## style[0]

```css

  @container style(        --foo:bar) { }
  @container STyle(--foo:    ) { }
  @container STyle(--foo:) { }
  @container STyle(--foo) { }
  @container  style(  ( --FOO: BAR) OR ( prop: val  ) ) { }
  @container style (--foo: bar) { }
  @container style(--foo: bar   baz) { }
  @container style(--foo:2.100  ) { }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
