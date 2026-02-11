# css/css-namespaces/syntax-008.xml

```json
{
  "format_version": 3,
  "file": "css/css-namespaces/syntax-008.xml"
}
```

## style[0]

```css

   @namespace \72x url("test");
   t { background:red }
   r\78|t { background:lime }
  
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

   @namespace \032  url("test"); /* two spaces, see CSS 2.1, 4.1.3 */
   t2 { background:red }
   \32|t2 { background:lime }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
