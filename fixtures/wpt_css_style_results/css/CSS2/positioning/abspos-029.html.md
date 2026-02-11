# css/CSS2/positioning/abspos-029.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/abspos-029.html"
}
```

## style[0]

```css

/* Set Up Conditions */
* { margin: 0; padding: 0; border: 0;
    position: static; float: none; display: block;
    top: auto; left: auto; right: auto; bottom: auto; }
head { display: none; }
body { padding: 1em; }

/* Test */
.inline { display: inline }
.margin { margin-bottom: 2em; }
.abs { position: absolute; background: green; z-index: 1; height: 1em; width: 10em; }
.flow { background: red; height: 1em; width: 10em; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
