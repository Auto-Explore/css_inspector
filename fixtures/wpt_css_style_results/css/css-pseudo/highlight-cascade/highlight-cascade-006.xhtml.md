# css/css-pseudo/highlight-cascade/highlight-cascade-006.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/highlight-cascade-006.xhtml"
}
```

## style[0]

```css

    main * { all: initial; display: block; }
    ::selection { color: green; } /* 1. universal (* means *|* if there is no default @namespace) */
    .red::selection { color: red; } /* 2. not universal; matches only .red */
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

    @namespace "http://example.org/default";
    @namespace foo "http://example.org/foo";
    *|*.bar::selection { color: red; } /* 3. not universal; matches only .bar */
    |*::selection { color: red; } /* 4. not universal; matches only no/empty xmlns */
    foo|*::selection { color: red; } /* 5. not universal; matches only xmlns http://example.org/foo */
    ::selection { color: red; } /* 6. not universal; matches only xmlns http://example.org/default */
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
