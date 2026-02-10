//! Static asset bundling

use std::path::Path;

use crate::error::{AnytronError, Result};

/// Asset bundler for CSS and JavaScript
pub struct AssetBundler;

impl AssetBundler {
    /// Create a new asset bundler
    pub fn new() -> Self {
        Self
    }

    /// Write the CSS stylesheet
    pub fn write_css(&self, output_path: &Path) -> Result<()> {
        std::fs::write(output_path, CSS_CONTENT).map_err(|e| AnytronError::FileWrite {
            path: output_path.to_path_buf(),
            source: e,
        })
    }

    /// Write the bundled JavaScript
    pub fn write_js(&self, output_path: &Path) -> Result<()> {
        let bundle = format!("{}\n{}\n{}", LUNR_JS_MINIFIED, SEARCH_JS, MEME_JS);

        std::fs::write(output_path, bundle).map_err(|e| AnytronError::FileWrite {
            path: output_path.to_path_buf(),
            source: e,
        })
    }
}

impl Default for AssetBundler {
    fn default() -> Self {
        Self::new()
    }
}

/// CSS stylesheet content
const CSS_CONTENT: &str = r#"/* Anytron - Quote Search & Meme Generator Styles */

:root {
    --color-bg: #1a1a2e;
    --color-bg-secondary: #16213e;
    --color-accent: #e94560;
    --color-text: #eee;
    --color-text-muted: #888;
    --color-border: #333;
    --font-main: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    --font-mono: 'Fira Code', 'Consolas', monospace;
    --spacing-xs: 0.25rem;
    --spacing-sm: 0.5rem;
    --spacing-md: 1rem;
    --spacing-lg: 2rem;
    --spacing-xl: 4rem;
    --border-radius: 8px;
    --shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
}

*, *::before, *::after {
    box-sizing: border-box;
}

html {
    font-size: 16px;
}

body {
    margin: 0;
    padding: 0;
    font-family: var(--font-main);
    background-color: var(--color-bg);
    color: var(--color-text);
    line-height: 1.6;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
}

a {
    color: var(--color-accent);
    text-decoration: none;
}

a:hover {
    text-decoration: underline;
}

/* Header */
.header {
    background-color: var(--color-bg-secondary);
    padding: var(--spacing-lg);
    text-align: center;
    border-bottom: 1px solid var(--color-border);
}

.header__title {
    margin: 0;
    font-size: 2rem;
    font-weight: 700;
}

.header__subtitle {
    margin: var(--spacing-sm) 0 0;
    color: var(--color-text-muted);
}

.header__back {
    display: inline-block;
    margin-bottom: var(--spacing-md);
    font-size: 0.875rem;
}

/* Main */
.main {
    flex: 1;
    padding: var(--spacing-lg);
    max-width: 1200px;
    margin: 0 auto;
    width: 100%;
}

/* Search Section */
.search-section {
    margin-bottom: var(--spacing-xl);
}

.search-form {
    display: flex;
    gap: var(--spacing-sm);
    max-width: 600px;
    margin: 0 auto;
}

.search-input {
    flex: 1;
    padding: var(--spacing-md);
    font-size: 1rem;
    border: 2px solid var(--color-border);
    border-radius: var(--border-radius);
    background-color: var(--color-bg-secondary);
    color: var(--color-text);
    transition: border-color 0.2s;
}

.search-input:focus {
    outline: none;
    border-color: var(--color-accent);
}

.search-input::placeholder {
    color: var(--color-text-muted);
}

.search-button {
    padding: var(--spacing-md) var(--spacing-lg);
    font-size: 1rem;
    font-weight: 600;
    border: none;
    border-radius: var(--border-radius);
    background-color: var(--color-accent);
    color: white;
    cursor: pointer;
    transition: opacity 0.2s;
}

.search-button:hover {
    opacity: 0.9;
}

/* Results Section */
.results-section {
    min-height: 200px;
}

.results-info {
    text-align: center;
    margin-bottom: var(--spacing-lg);
    color: var(--color-text-muted);
}

.results-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: var(--spacing-md);
}

/* Result Card */
.result-card {
    background-color: var(--color-bg-secondary);
    border-radius: var(--border-radius);
    overflow: hidden;
    box-shadow: var(--shadow);
    transition: transform 0.2s, box-shadow 0.2s;
}

.result-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.4);
}

.result-card__link {
    display: block;
    color: inherit;
    text-decoration: none;
}

.result-card__image {
    width: 100%;
    aspect-ratio: 16/9;
    object-fit: cover;
    display: block;
}

.result-card__content {
    padding: var(--spacing-md);
}

.result-card__text {
    margin: 0 0 var(--spacing-sm);
    font-size: 0.875rem;
    line-height: 1.4;
    /* Clamp to 3 lines */
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
}

.result-card__meta {
    display: flex;
    justify-content: space-between;
    font-size: 0.75rem;
    color: var(--color-text-muted);
}

/* Caption Page */
.caption-page {
    max-width: 900px;
}

.caption-section {
    background-color: var(--color-bg-secondary);
    border-radius: var(--border-radius);
    overflow: hidden;
    margin-bottom: var(--spacing-lg);
}

.caption-image-container {
    position: relative;
}

.caption-image {
    width: 100%;
    display: block;
}

.caption-overlay {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    background: linear-gradient(transparent, rgba(0, 0, 0, 0.8));
    padding: var(--spacing-xl) var(--spacing-lg) var(--spacing-lg);
    text-align: center;
    pointer-events: none;
}

.caption-text {
    font-size: 1.25rem;
    font-weight: 600;
    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.8);
}

.caption-info {
    padding: var(--spacing-lg);
}

.caption-quote {
    font-size: 1.25rem;
    font-style: italic;
    margin: 0 0 var(--spacing-md);
}

.caption-meta {
    display: flex;
    gap: var(--spacing-lg);
    color: var(--color-text-muted);
    font-size: 0.875rem;
}

/* Meme Controls */
.meme-controls {
    padding: var(--spacing-lg);
    border-top: 1px solid var(--color-border);
}

.meme-controls h3 {
    margin: 0 0 var(--spacing-md);
    font-size: 1rem;
}

.meme-form {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
}

.meme-textarea {
    width: 100%;
    min-height: 80px;
    padding: var(--spacing-md);
    font-size: 1rem;
    border: 2px solid var(--color-border);
    border-radius: var(--border-radius);
    background-color: var(--color-bg);
    color: var(--color-text);
    resize: vertical;
}

.meme-textarea:focus {
    outline: none;
    border-color: var(--color-accent);
}

.meme-options {
    display: flex;
    flex-wrap: wrap;
    gap: var(--spacing-lg);
    align-items: center;
}

.meme-options label {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    font-size: 0.875rem;
}

.meme-buttons {
    display: flex;
    gap: var(--spacing-sm);
    flex-wrap: wrap;
}

.meme-button {
    padding: var(--spacing-md) var(--spacing-lg);
    font-size: 1rem;
    font-weight: 600;
    border: none;
    border-radius: var(--border-radius);
    background-color: var(--color-accent);
    color: white;
    cursor: pointer;
    transition: opacity 0.2s;
    flex: 1;
    min-width: 120px;
}

.meme-button:hover {
    opacity: 0.9;
}

/* Caption Navigation */
.caption-nav {
    display: flex;
    justify-content: space-between;
    gap: var(--spacing-md);
}

.caption-nav__link {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-md);
    background-color: var(--color-bg-secondary);
    border-radius: var(--border-radius);
    color: inherit;
    text-decoration: none;
    transition: background-color 0.2s;
}

.caption-nav__link:hover {
    background-color: var(--color-border);
    text-decoration: none;
}

.caption-nav__thumb {
    width: 80px;
    height: 45px;
    object-fit: cover;
    border-radius: 4px;
}

.caption-nav__disabled {
    visibility: hidden;
}

/* Footer */
.footer {
    padding: var(--spacing-lg);
    text-align: center;
    color: var(--color-text-muted);
    font-size: 0.875rem;
    border-top: 1px solid var(--color-border);
}

/* Loading State */
.loading {
    text-align: center;
    padding: var(--spacing-xl);
}

.loading::after {
    content: '';
    display: inline-block;
    width: 24px;
    height: 24px;
    border: 3px solid var(--color-border);
    border-top-color: var(--color-accent);
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

@keyframes spin {
    to { transform: rotate(360deg); }
}

/* Responsive */
@media (max-width: 768px) {
    .header__title {
        font-size: 1.5rem;
    }

    .search-form {
        flex-direction: column;
    }

    .search-button {
        width: 100%;
    }

    .results-grid {
        grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    }

    .caption-nav {
        flex-direction: column;
    }
}
"#;

/// Minified lunr.js library (v2.3.9)
/// Source: <https://lunrjs.com/>
const LUNR_JS_MINIFIED: &str = r#"/**
 * lunr - http://lunrjs.com - A bit like Solr, but much smaller and not as bright - 2.3.9
 * Copyright (C) 2020 Oliver Nightingale
 * @license MIT
 */
!function(){var e=function(t){var r=new e.Builder;return r.pipeline.add(e.trimmer,e.stopWordFilter,e.stemmer),r.searchPipeline.add(e.stemmer),t.call(r,r),r.build()};e.version="2.3.9",e.utils={},e.utils.warn=function(e){return function(t){e.console&&console.warn&&console.warn(t)}}(this),e.utils.asString=function(e){return void 0===e||null===e?"":e.toString()},e.utils.clone=function(e){if(null===e||void 0===e)return e;for(var t=Object.create(null),r=Object.keys(e),n=0;n<r.length;n++){var i=r[n],s=e[i];if(Array.isArray(s))t[i]=s.slice();else{if("string"!=typeof s&&"number"!=typeof s&&"boolean"!=typeof s)throw new TypeError("clone is not deep and does not support nested objects");t[i]=s}}return t},e.FieldRef=function(e,t,r){this.docRef=e,this.fieldName=t,this._stringValue=r},e.FieldRef.joiner="/",e.FieldRef.fromString=function(t){var r=t.indexOf(e.FieldRef.joiner);if(-1===r)throw"malformed field ref string";var n=t.slice(0,r),i=t.slice(r+1);return new e.FieldRef(i,n,t)},e.FieldRef.prototype.toString=function(){return void 0==this._stringValue&&(this._stringValue=this.fieldName+e.FieldRef.joiner+this.docRef),this._stringValue},e.Set=function(e){if(this.elements=Object.create(null),e){this.length=e.length;for(var t=0;t<this.length;t++)this.elements[e[t]]=!0}else this.length=0},e.Set.complete={intersect:function(e){return e},union:function(){return this},contains:function(){return!0}},e.Set.empty={intersect:function(){return this},union:function(e){return e},contains:function(){return!1}},e.Set.prototype.contains=function(e){return!!this.elements[e]},e.Set.prototype.intersect=function(t){var r,n,i,s=[];if(t===e.Set.complete)return this;if(t===e.Set.empty)return t;this.length<t.length?(r=this,n=t):(r=t,n=this),i=Object.keys(r.elements);for(var o=0;o<i.length;o++){var a=i[o];a in n.elements&&s.push(a)}return new e.Set(s)},e.Set.prototype.union=function(t){return t===e.Set.complete?e.Set.complete:t===e.Set.empty?this:new e.Set(Object.keys(this.elements).concat(Object.keys(t.elements)))},e.idf=function(e,t){var r=0;for(var n in e)"_index"!=n&&(r+=Object.keys(e[n]).length);var i=(t-r+.5)/(r+.5);return i<1&&(i=1e-10),Math.log(1+i)},e.Token=function(e,t){this.str=e||"",this.metadata=t||{}},e.Token.prototype.toString=function(){return this.str},e.Token.prototype.update=function(e){return this.str=e(this.str,this.metadata),this},e.Token.prototype.clone=function(t){return t=t||function(e){return e},new e.Token(t(this.str,this.metadata),this.metadata)},e.tokenizer=function(t,r){if(null==t||void 0==t)return[];if(Array.isArray(t))return t.map((function(t){return new e.Token(e.utils.asString(t).toLowerCase(),e.utils.clone(r))}));for(var n=t.toString().toLowerCase(),i=n.length,s=[],o=0,a=0;o<=i;o++){var u=o-a;if(n.charAt(o).match(e.tokenizer.separator)||o==i){if(u>0){var l=e.utils.clone(r)||{};l.position=[a,u],l.index=s.length,s.push(new e.Token(n.slice(a,o),l))}a=o+1}}return s},e.tokenizer.separator=/[\s\-]+/,e.Pipeline=function(){this._stack=[]},e.Pipeline.registeredFunctions=Object.create(null),e.Pipeline.registerFunction=function(t,r){r in this.registeredFunctions&&e.utils.warn("Overwriting existing registered function: "+r),t.label=r,e.Pipeline.registeredFunctions[t.label]=t},e.Pipeline.warnIfFunctionNotRegistered=function(t){t.label&&t.label in this.registeredFunctions||e.utils.warn("Function is not registered with pipeline. This may cause problems when serialising the index.\n",t)},e.Pipeline.load=function(t){var r=new e.Pipeline;return t.forEach((function(t){var n=e.Pipeline.registeredFunctions[t];if(!n)throw new Error("Cannot load unregistered function: "+t);r.add(n)})),r},e.Pipeline.prototype.add=function(){Array.prototype.slice.call(arguments).forEach((function(t){e.Pipeline.warnIfFunctionNotRegistered(t),this._stack.push(t)}),this)},e.Pipeline.prototype.after=function(t,r){e.Pipeline.warnIfFunctionNotRegistered(r);var n=this._stack.indexOf(t);if(-1==n)throw new Error("Cannot find existingFn");n+=1,this._stack.splice(n,0,r)},e.Pipeline.prototype.before=function(t,r){e.Pipeline.warnIfFunctionNotRegistered(r);var n=this._stack.indexOf(t);if(-1==n)throw new Error("Cannot find existingFn");this._stack.splice(n,0,r)},e.Pipeline.prototype.remove=function(e){var t=this._stack.indexOf(e);-1!=t&&this._stack.splice(t,1)},e.Pipeline.prototype.run=function(e){for(var t=this._stack.length,r=0;r<t;r++){for(var n=this._stack[r],i=[],s=0;s<e.length;s++){var o=n(e[s],s,e);if(void 0!==o&&""!==o)if(Array.isArray(o))for(var a=0;a<o.length;a++)i.push(o[a]);else i.push(o)}e=i}return e},e.Pipeline.prototype.runString=function(t,r){var n=new e.Token(t,r);return this.run([n]).map((function(e){return e.toString()}))},e.Pipeline.prototype.reset=function(){this._stack=[]},e.Pipeline.prototype.toJSON=function(){return this._stack.map((function(t){return e.Pipeline.warnIfFunctionNotRegistered(t),t.label}))},e.Vector=function(e){this._magnitude=0,this.elements=e||[]},e.Vector.prototype.positionForIndex=function(e){if(0==this.elements.length)return 0;for(var t=0,r=this.elements.length/2,n=r-t,i=Math.floor(n/2),s=this.elements[2*i];n>1&&(s<e&&(t=i),s>e&&(r=i),s!=e);)n=r-t,i=t+Math.floor(n/2),s=this.elements[2*i];return s==e||s>e?2*i:s<e?2*(i+1):void 0},e.Vector.prototype.insert=function(e,t){this.upsert(e,t,(function(){throw"duplicate index"}))},e.Vector.prototype.upsert=function(e,t,r){this._magnitude=0;var n=this.positionForIndex(e);this.elements[n]==e?this.elements[n+1]=r(this.elements[n+1],t):this.elements.splice(n,0,e,t)},e.Vector.prototype.magnitude=function(){if(this._magnitude)return this._magnitude;for(var e=0,t=this.elements.length,r=1;r<t;r+=2){var n=this.elements[r];e+=n*n}return this._magnitude=Math.sqrt(e)},e.Vector.prototype.dot=function(e){for(var t=0,r=this.elements,n=e.elements,i=r.length,s=n.length,o=0,a=0,u=0,l=0;u<i&&l<s;)(o=r[u])<(a=n[l])?u+=2:o>a?l+=2:o==a&&(t+=r[u+1]*n[l+1],u+=2,l+=2);return t},e.Vector.prototype.similarity=function(e){return this.dot(e)/this.magnitude()||0},e.Vector.prototype.toArray=function(){for(var e=new Array(this.elements.length/2),t=1,r=0;t<this.elements.length;t+=2,r++)e[r]=this.elements[t];return e},e.Vector.prototype.toJSON=function(){return this.elements},e.stemmer=function(){var e={ational:"ate",tional:"tion",enci:"ence",anci:"ance",izer:"ize",bli:"ble",alli:"al",entli:"ent",eli:"e",ousli:"ous",ization:"ize",ation:"ate",ator:"ate",alism:"al",iveness:"ive",fulness:"ful",ousness:"ous",aliti:"al",iviti:"ive",biliti:"ble",logi:"log"},t={icate:"ic",ative:"",alize:"al",iciti:"ic",ical:"ic",ful:"",ness:""},r="[^aeiou]",n="[aeiouy]",i=r+"[^aeiouy]*",s=n+"[aeiou]*",o="^("+i+")?"+s+i,a="^("+i+")?"+s+i+"("+s+")?$",u="^("+i+")?"+s+i+s+i,l="^("+i+")?"+n,c=new RegExp(o),d=new RegExp(u),h=new RegExp(a),f=new RegExp(l),p=/^(.+?)(ss|i)es$/,m=/^(.+?)([^s])s$/,y=/^(.+?)eed$/,v=/^(.+?)(ed|ing)$/,g=/.$/,w=/(at|bl|iz)$/,x=/([^aeiouylsz])\1$/,k=new RegExp("^"+i+n+"[^aeiouwxy]$"),S=/^(.+?[^aeiou])y$/,b=/^(.+?)(ational|tional|enci|anci|izer|bli|alli|entli|eli|ousli|ization|ation|ator|alism|iveness|fulness|ousness|aliti|iviti|biliti|logi)$/,E=/^(.+?)(icate|ative|alize|iciti|ical|ful|ness)$/,L=/^(.+?)(al|ance|ence|er|ic|able|ible|ant|ement|ment|ent|ou|ism|ate|iti|ous|ive|ize)$/,P=/^(.+?)(s|t)(ion)$/,T=/^(.+?)e$/,O=/ll$/,I=new RegExp("^"+i+n+"[^aeiouwxy]$"),R=function(r){var n,i,s,o,a,u,l;if(r.length<3)return r;if("y"==(s=r.substr(0,1))&&(r=s.toUpperCase()+r.substr(1)),a=m,(o=p).test(r)?r=r.replace(o,"$1$2"):a.test(r)&&(r=r.replace(a,"$1$2")),a=v,(o=y).test(r)){var R=o.exec(r);(o=c).test(R[1])&&(o=g,r=r.replace(o,""))}else if(a.test(r)){n=(R=a.exec(r))[1],(a=f).test(n)&&(u=x,l=k,(a=w).test(r=n)?r+="e":u.test(r)?(o=g,r=r.replace(o,"")):l.test(r)&&(r+="e"))}if((o=S).test(r)&&(r=(n=(R=o.exec(r))[1])+"i"),(o=b).test(r)&&(n=(R=o.exec(r))[1],i=R[2],(o=c).test(n)&&(r=n+e[i])),(o=E).test(r)&&(n=(R=o.exec(r))[1],i=R[2],(o=c).test(n)&&(r=n+t[i])),(o=L).test(r))n=(R=o.exec(r))[1],(o=d).test(n)&&(r=n);else if((o=P).test(r)&&(n=(R=o.exec(r))[1]+R[2],(o=d).test(n)&&(r=n)));return(o=T).test(r)&&(n=(R=o.exec(r))[1],u=h,l=I,((o=d).test(n)||u.test(n)&&!l.test(n))&&(r=n)),(o=O).test(r)&&(o=d).test(r)&&(o=g,r=r.replace(o,"")),"y"==s&&(r=s.toLowerCase()+r.substr(1)),r};return function(t){return t.update(R)}}(),e.Pipeline.registerFunction(e.stemmer,"stemmer"),e.generateStopWordFilter=function(t){var r=t.reduce((function(e,t){return e[t]=t,e}),{});return function(t){if(t&&r[t.toString()]!==t.toString())return t}},e.stopWordFilter=e.generateStopWordFilter(["a","able","about","across","after","all","almost","also","am","among","an","and","any","are","as","at","be","because","been","but","by","can","cannot","could","dear","did","do","does","either","else","ever","every","for","from","get","got","had","has","have","he","her","hers","him","his","how","however","i","if","in","into","is","it","its","just","least","let","like","likely","may","me","might","most","must","my","neither","no","nor","not","of","off","often","on","only","or","other","our","own","rather","said","say","says","she","should","since","so","some","than","that","the","their","them","then","there","these","they","this","tis","to","too","twas","us","wants","was","we","were","what","when","where","which","while","who","whom","why","will","with","would","yet","you","your"]),e.Pipeline.registerFunction(e.stopWordFilter,"stopWordFilter"),e.trimmer=function(e){return e.update((function(e){return e.replace(/^\W+/,"").replace(/\W+$/,"")}))},e.Pipeline.registerFunction(e.trimmer,"trimmer"),e.TokenSet=function(){this.final=!1,this.edges={},this.id=e.TokenSet._nextId,e.TokenSet._nextId+=1},e.TokenSet._nextId=1,e.TokenSet.fromArray=function(t){for(var r=new e.TokenSet.Builder,n=0,i=t.length;n<i;n++)r.insert(t[n]);return r.finish(),r.root},e.TokenSet.fromClause=function(t){"leading"in t&&(e.utils.warn("Warning: Leading wildcards are not supported and will be ignored"),delete t.leading),"trailing"in t&&(e.utils.warn("Warning: Trailing wildcards are not supported and will be ignored"),delete t.trailing);var r=new e.TokenSet.Builder;return r.insert(t.term),r.root},e.TokenSet.fromFuzzyString=function(t,r){for(var n=new e.TokenSet,i=[{node:n,editsRemaining:r,str:t}];i.length;){var s=i.pop();if(s.str.length>0){var o,a=s.str.charAt(0);a in s.node.edges?o=s.node.edges[a]:(o=new e.TokenSet,s.node.edges[a]=o),1==s.str.length&&(o.final=!0),i.push({node:o,editsRemaining:s.editsRemaining,str:s.str.slice(1)})}if(0!=s.editsRemaining){if("*"in s.node.edges)var u=s.node.edges["*"];else{u=new e.TokenSet;s.node.edges["*"]=u}if(0==s.str.length&&(u.final=!0),i.push({node:u,editsRemaining:s.editsRemaining-1,str:s.str}),s.str.length>1&&i.push({node:s.node,editsRemaining:s.editsRemaining-1,str:s.str.slice(1)}),1==s.str.length&&(s.node.final=!0),s.str.length>=1){if("*"in s.node.edges)var l=s.node.edges["*"];else{l=new e.TokenSet;s.node.edges["*"]=l}1==s.str.length&&(l.final=!0),i.push({node:l,editsRemaining:s.editsRemaining-1,str:s.str.slice(1)})}if(s.str.length>1){var c,d=s.str.charAt(0),h=s.str.charAt(1);h in s.node.edges?c=s.node.edges[h]:(c=new e.TokenSet,s.node.edges[h]=c),1==s.str.length&&(c.final=!0),i.push({node:c,editsRemaining:s.editsRemaining-1,str:d+s.str.slice(2)})}}}return n},e.TokenSet.fromString=function(t){for(var r=new e.TokenSet,n=r,i=0,s=t.length;i<s;i++){var o=t[i],a=i==s-1;if("*"==o)r.edges[o]=r,r.final=a;else{var u=new e.TokenSet;u.final=a,r.edges[o]=u,r=u}}return n},e.TokenSet.prototype.toArray=function(){for(var e=[],t=[{prefix:"",node:this}];t.length;){var r=t.pop(),n=Object.keys(r.node.edges),i=n.length;if(r.node.final&&(r.prefix.length>0||i==0)&&e.push(r.prefix),i)for(var s=0;s<i;s++){var o=n[s];t.push({prefix:r.prefix.concat(o),node:r.node.edges[o]})}}return e},e.TokenSet.prototype.toString=function(){if(this._str)return this._str;for(var e=this.final?"1":"0",t=Object.keys(this.edges).sort(),r=t.length,n=0;n<r;n++){var i=t[n];e=e+i+this.edges[i].id}return e},e.TokenSet.prototype.intersect=function(t){for(var r=new e.TokenSet,n=void 0,i=[{qNode:t,output:r,node:this}];i.length;){var s=i.pop(),o=Object.keys(s.qNode.edges),a=o.length,u=Object.keys(s.node.edges),l=u.length;for(n=0;n<a;n++)for(var c=o[n],d=0;d<l;d++){var h=u[d];if(h==c||"*"==c){var f=s.node.edges[h],p=s.qNode.edges[c],m=f.final&&p.final,y=void 0;h in s.output.edges?(y=s.output.edges[h]).final=y.final||m:((y=new e.TokenSet).final=m,s.output.edges[h]=y),i.push({qNode:p,output:y,node:f})}}}return r},e.TokenSet.Builder=function(){this.previousWord="",this.root=new e.TokenSet,this.uncheckedNodes=[],this.minimizedNodes={}},e.TokenSet.Builder.prototype.insert=function(t){var r,n=0;if(t<this.previousWord)throw new Error("Out of order word insertion");for(;n<t.length&&n<this.previousWord.length&&t[n]==this.previousWord[n];)n++;this.minimize(n),r=0==this.uncheckedNodes.length?this.root:this.uncheckedNodes[this.uncheckedNodes.length-1].child;for(var i=n;i<t.length;i++){var s=new e.TokenSet,o=t[i];r.edges[o]=s,this.uncheckedNodes.push({parent:r,char:o,child:s}),r=s}r.final=!0,this.previousWord=t},e.TokenSet.Builder.prototype.finish=function(){this.minimize(0)},e.TokenSet.Builder.prototype.minimize=function(e){for(var t=this.uncheckedNodes.length-1;t>=e;t--){var r=this.uncheckedNodes[t],n=r.child.toString();n in this.minimizedNodes?r.parent.edges[r.char]=this.minimizedNodes[n]:(r.child._str=n,this.minimizedNodes[n]=r.child),this.uncheckedNodes.pop()}},e.Index=function(e){this.invertedIndex=e.invertedIndex,this.fieldVectors=e.fieldVectors,this.tokenSet=e.tokenSet,this.fields=e.fields,this.pipeline=e.pipeline},e.Index.prototype.search=function(t){return this.query((function(r){new e.QueryParser(t,r).parse()}))},e.Index.prototype.query=function(t){for(var r=new e.Query(this.fields),n=Object.create(null),i=Object.create(null),s=Object.create(null),o=Object.create(null),a=Object.create(null),u=0;u<this.fields.length;u++)i[this.fields[u]]=new e.Vector;t.call(r,r);for(u=0;u<r.clauses.length;u++){var l=r.clauses[u],c=null,d=e.Set.empty;c=l.usePipeline?this.pipeline.runString(l.term,{fields:l.fields}):[l.term];for(var h=0;h<c.length;h++){var f=c[h];l.term=f;var p=e.TokenSet.fromClause(l),m=this.tokenSet.intersect(p).toArray();if(0===m.length&&l.presence===e.Query.presence.REQUIRED){for(var y=0;y<l.fields.length;y++){o[W=l.fields[y]]=e.Set.empty}break}for(var v=0;v<m.length;v++){var g=m[v],w=this.invertedIndex[g],x=w._index;for(y=0;y<l.fields.length;y++){var k=w[W=l.fields[y]],S=Object.keys(k),b=g+"/"+W,E=new e.Set(S);if(d=d.union(E),l.presence==e.Query.presence.REQUIRED&&(a[W]=a[W]?a[W].union(E):E),l.presence!=e.Query.presence.PROHIBITED){if(i[W].upsert(x,l.boost,(function(e,t){return e+t})),!s[b]){for(var L=0;L<S.length;L++){var P,T=S[L],O=new e.FieldRef(T,W),I=k[T];(P=n[O])===void 0?n[O]=new e.MatchData(g,W,I):P.add(g,W,I)}s[b]=!0}}else void 0===o[W]&&(o[W]=e.Set.complete)}}}if(l.presence===e.Query.presence.REQUIRED)for(y=0;y<l.fields.length;y++){var W;o[W=l.fields[y]]=o[W].intersect(d)}}for(var R=e.Set.complete,F=e.Set.empty,Q=0;Q<this.fields.length;Q++){var W=this.fields[Q];a[W]&&(R=R.intersect(a[W])),o[W]&&(F=F.union(o[W]))}var N=Object.keys(n),C=[],j=Object.create(null);if(r.isNegated()){N=Object.keys(this.fieldVectors);for(u=0;u<N.length;u++){O=N[u];var D=e.FieldRef.fromString(O);n[O]=new e.MatchData}}for(u=0;u<N.length;u++){var _=(D=e.FieldRef.fromString(N[u])).docRef;if(R.contains(_)&&!F.contains(_)){var M,A=this.fieldVectors[D],B=i[D.fieldName].similarity(A);if((M=j[_])!==void 0)M.score+=B,M.matchData.combine(n[D]);else{var U={ref:_,score:B,matchData:n[D]};j[_]=U,C.push(U)}}}return C.sort((function(e,t){return t.score-e.score}))},e.Index.prototype.toJSON=function(){var t=Object.keys(this.invertedIndex).sort().map((function(e){return[e,this.invertedIndex[e]]}),this),r=Object.keys(this.fieldVectors).map((function(e){return[e,this.fieldVectors[e].toJSON()]}),this);return{version:e.version,fields:this.fields,fieldVectors:r,invertedIndex:t,pipeline:this.pipeline.toJSON()}},e.Index.load=function(t){var r={},n={},i=t.fieldVectors,s=Object.create(null),o=t.invertedIndex,a=new e.TokenSet.Builder,u=e.Pipeline.load(t.pipeline);t.version!=e.version&&e.utils.warn("Version mismatch when loading serialised index. Current version of lunr '"+e.version+"' does not match serialized index '"+t.version+"'");for(var l=0;l<i.length;l++){var c=(d=i[l])[0],h=d[1];n[c]=new e.Vector(h)}for(l=0;l<o.length;l++){var d,f=(d=o[l])[0],p=d[1];a.insert(f),s[f]=p}return a.finish(),r.fields=t.fields,r.fieldVectors=n,r.invertedIndex=s,r.tokenSet=a.root,r.pipeline=u,new e.Index(r)},e.Builder=function(){this._ref="id",this._fields=Object.create(null),this._documents=Object.create(null),this.invertedIndex=Object.create(null),this.fieldTermFrequencies={},this.fieldLengths={},this.tokenizer=e.tokenizer,this.pipeline=new e.Pipeline,this.searchPipeline=new e.Pipeline,this.documentCount=0,this._b=.75,this._k1=1.2,this.termIndex=0,this.metadataWhitelist=[]},e.Builder.prototype.ref=function(e){this._ref=e},e.Builder.prototype.field=function(e,t){if(/\//.test(e))throw new RangeError("Field '"+e+"' contains illegal character '/'");this._fields[e]=t||{}},e.Builder.prototype.b=function(e){this._b=e<0?0:e>1?1:e},e.Builder.prototype.k1=function(e){this._k1=e},e.Builder.prototype.add=function(t,r){var n=t[this._ref],i=Object.keys(this._fields);this._documents[n]=r||{},this.documentCount+=1;for(var s=0;s<i.length;s++){var o=i[s],a=this._fields[o].extractor,u=a?a(t):t[o],l=this.tokenizer(u,{fields:[o]}),c=this.pipeline.run(l),d=new e.FieldRef(n,o),h=Object.create(null);this.fieldTermFrequencies[d]=h,this.fieldLengths[d]=0,this.fieldLengths[d]+=c.length;for(var f=0;f<c.length;f++){var p=c[f];if(null==h[p]&&(h[p]=0),h[p]+=1,null==this.invertedIndex[p]){var m=Object.create(null);m._index=this.termIndex,this.termIndex+=1;for(var y=0;y<i.length;y++)m[i[y]]=Object.create(null);this.invertedIndex[p]=m}null==this.invertedIndex[p][o][n]&&(this.invertedIndex[p][o][n]=Object.create(null));for(var v=0;v<this.metadataWhitelist.length;v++){var g=this.metadataWhitelist[v],w=p.metadata[g];null==this.invertedIndex[p][o][n][g]&&(this.invertedIndex[p][o][n][g]=[]),this.invertedIndex[p][o][n][g].push(w)}}}},e.Builder.prototype.calculateAverageFieldLengths=function(){for(var t=Object.keys(this.fieldLengths),r=t.length,n={},i={},s=0;s<r;s++){var o=e.FieldRef.fromString(t[s]),a=o.fieldName;i[a]||(i[a]=0),i[a]+=1,n[a]||(n[a]=0),n[a]+=this.fieldLengths[o]}var u=Object.keys(this._fields);for(s=0;s<u.length;s++){var l=u[s];n[l]=n[l]/i[l]}this.averageFieldLength=n},e.Builder.prototype.createFieldVectors=function(){for(var t={},r=Object.keys(this.fieldTermFrequencies),n=r.length,i=Object.create(null),s=0;s<n;s++){for(var o=e.FieldRef.fromString(r[s]),a=o.fieldName,u=this.fieldLengths[o],l=new e.Vector,c=this.fieldTermFrequencies[o],d=Object.keys(c),h=d.length,f=this._fields[a].boost||1,p=this._documents[o.docRef].boost||1,m=0;m<h;m++){var y,v,g,w=d[m],x=c[w],k=this.invertedIndex[w]._index;void 0===i[w]?(y=e.idf(this.invertedIndex[w],this.documentCount),i[w]=y):y=i[w],v=y*((this._k1+1)*x)/(this._k1*(1-this._b+this._b*(u/this.averageFieldLength[a]))+x),v*=f,v*=p,g=Math.round(1e3*v)/1e3,l.insert(k,g)}t[o]=l}this.fieldVectors=t},e.Builder.prototype.createTokenSet=function(){this.tokenSet=e.TokenSet.fromArray(Object.keys(this.invertedIndex).sort())},e.Builder.prototype.build=function(){return this.calculateAverageFieldLengths(),this.createFieldVectors(),this.createTokenSet(),new e.Index({invertedIndex:this.invertedIndex,fieldVectors:this.fieldVectors,tokenSet:this.tokenSet,fields:Object.keys(this._fields),pipeline:this.searchPipeline})},e.Builder.prototype.use=function(e){var t=Array.prototype.slice.call(arguments,1);t.unshift(this),e.apply(this,t)},e.MatchData=function(e,t,r){for(var n=Object.create(null),i=Object.keys(r||{}),s=0;s<i.length;s++){var o=i[s];n[o]=r[o].slice()}this.metadata=Object.create(null),void 0!==e&&(this.metadata[e]=Object.create(null),this.metadata[e][t]=n)},e.MatchData.prototype.combine=function(e){for(var t=Object.keys(e.metadata),r=0;r<t.length;r++){var n=t[r],i=Object.keys(e.metadata[n]);void 0==this.metadata[n]&&(this.metadata[n]=Object.create(null));for(var s=0;s<i.length;s++){var o=i[s],a=Object.keys(e.metadata[n][o]);void 0==this.metadata[n][o]&&(this.metadata[n][o]=Object.create(null));for(var u=0;u<a.length;u++){var l=a[u];void 0==this.metadata[n][o][l]?this.metadata[n][o][l]=e.metadata[n][o][l].slice():this.metadata[n][o][l]=this.metadata[n][o][l].concat(e.metadata[n][o][l])}}}},e.Query=function(e){this.clauses=[],this.allFields=e},e.Query.wildcard=new String("*"),e.Query.wildcard.NONE=0,e.Query.wildcard.LEADING=1,e.Query.wildcard.TRAILING=2,e.Query.presence={OPTIONAL:1,REQUIRED:2,PROHIBITED:3},e.Query.prototype.clause=function(t){return"fields"in t||(t.fields=this.allFields),"boost"in t||(t.boost=1),"usePipeline"in t||(t.usePipeline=!0),"wildcard"in t||(t.wildcard=e.Query.wildcard.NONE),t.wildcard&e.Query.wildcard.LEADING&&t.term.charAt(0)!=e.Query.wildcard&&(t.term="*"+t.term),t.wildcard&e.Query.wildcard.TRAILING&&t.term.slice(-1)!=e.Query.wildcard&&(t.term=t.term+"*"),"presence"in t||(t.presence=e.Query.presence.OPTIONAL),this.clauses.push(t),this},e.Query.prototype.isNegated=function(){for(var t=0;t<this.clauses.length;t++)if(this.clauses[t].presence!=e.Query.presence.PROHIBITED)return!1;return!0},e.Query.prototype.term=function(t,r){if(Array.isArray(t))return t.forEach((function(t){this.term(t,e.utils.clone(r))}),this),this;var n=r||{};return n.term=t.toString(),this.clause(n),this},e.QueryParseError=function(e,t,r){this.name="QueryParseError",this.message=e,this.start=t,this.end=r},e.QueryParseError.prototype=new Error,e.QueryLexer=function(e){this.lexemes=[],this.str=e,this.length=e.length,this.pos=0,this.start=0,this.escapeCharPositions=[]},e.QueryLexer.prototype.run=function(){for(var t=e.QueryLexer.lexText;t;)t=t(this)},e.QueryLexer.prototype.sliceString=function(){for(var e=[],t=this.start,r=this.pos,n=0;n<this.escapeCharPositions.length;n++)r=this.escapeCharPositions[n],e.push(this.str.slice(t,r)),t=r+1;return e.push(this.str.slice(t,this.pos)),this.escapeCharPositions.length=0,e.join("")},e.QueryLexer.prototype.emit=function(e){this.lexemes.push({type:e,str:this.sliceString(),start:this.start,end:this.pos}),this.start=this.pos},e.QueryLexer.prototype.escapeCharacter=function(){this.escapeCharPositions.push(this.pos-1),this.pos+=1},e.QueryLexer.prototype.next=function(){if(this.pos<this.length)return this.str.charAt(this.pos++)},e.QueryLexer.prototype.width=function(){return this.pos-this.start},e.QueryLexer.prototype.ignore=function(){this.start==this.pos&&(this.pos+=1),this.start=this.pos},e.QueryLexer.prototype.backup=function(){this.pos-=1},e.QueryLexer.prototype.acceptDigitRun=function(){var t,r;do{r=(t=this.next())&&t.charCodeAt(0)}while(r>47&&r<58);t&&this.backup()},e.QueryLexer.prototype.more=function(){return this.pos<this.length},e.QueryLexer.EOS="EOS",e.QueryLexer.FIELD="FIELD",e.QueryLexer.TERM="TERM",e.QueryLexer.EDIT_DISTANCE="EDIT_DISTANCE",e.QueryLexer.BOOST="BOOST",e.QueryLexer.PRESENCE="PRESENCE",e.QueryLexer.lexField=function(t){return t.backup(),t.emit(e.QueryLexer.FIELD),t.ignore(),e.QueryLexer.lexText},e.QueryLexer.lexTerm=function(t){if(t.width()>1&&(t.backup(),t.emit(e.QueryLexer.TERM)),t.ignore(),t.more())return e.QueryLexer.lexText},e.QueryLexer.lexEditDistance=function(t){return t.ignore(),t.acceptDigitRun(),t.emit(e.QueryLexer.EDIT_DISTANCE),e.QueryLexer.lexText},e.QueryLexer.lexBoost=function(t){return t.ignore(),t.acceptDigitRun(),t.emit(e.QueryLexer.BOOST),e.QueryLexer.lexText},e.QueryLexer.lexEOS=function(t){t.width()>0&&t.emit(e.QueryLexer.TERM)},e.QueryLexer.lexText=function(t){for(;;){var r=t.next();if(null==r)return e.QueryLexer.lexEOS;if(92!=r.charCodeAt(0)){if(":"==r)return e.QueryLexer.lexField;if("~"==r)return t.backup(),t.width()>0&&t.emit(e.QueryLexer.TERM),e.QueryLexer.lexEditDistance;if("^"==r)return t.backup(),t.width()>0&&t.emit(e.QueryLexer.TERM),e.QueryLexer.lexBoost;if("+"==r&&1===t.width())return t.emit(e.QueryLexer.PRESENCE),e.QueryLexer.lexText;if("-"==r&&1===t.width())return t.emit(e.QueryLexer.PRESENCE),e.QueryLexer.lexText;if(r.match(e.QueryLexer.termSeparator))return e.QueryLexer.lexTerm}else t.escapeCharacter()}},e.QueryLexer.termSeparator=/[\s\-]+/,e.QueryParser=function(t,r){this.lexer=new e.QueryLexer(t),this.query=r,this.currentClause={},this.lexemeIdx=0},e.QueryParser.prototype.parse=function(){this.lexer.run(),this.lexemes=this.lexer.lexemes;for(var t=e.QueryParser.parseClause;t;)t=t(this);return this.query},e.QueryParser.prototype.peekLexeme=function(){return this.lexemes[this.lexemeIdx]},e.QueryParser.prototype.consumeLexeme=function(){var e=this.peekLexeme();return this.lexemeIdx+=1,e},e.QueryParser.prototype.nextClause=function(){var e=this.currentClause;this.query.clause(e),this.currentClause={}},e.QueryParser.parseClause=function(t){var r=t.peekLexeme();if(null!=r)switch(r.type){case e.QueryLexer.PRESENCE:return e.QueryParser.parsePresence;case e.QueryLexer.FIELD:return e.QueryParser.parseField;case e.QueryLexer.TERM:return e.QueryParser.parseTerm;default:var n="expected either a field or a term, found "+r.type;throw r.str.length>=1&&(n+=" with value '"+r.str+"'"),new e.QueryParseError(n,r.start,r.end)}},e.QueryParser.parsePresence=function(t){var r=t.consumeLexeme();if(null!=r){switch(r.str){case"-":t.currentClause.presence=e.Query.presence.PROHIBITED;break;case"+":t.currentClause.presence=e.Query.presence.REQUIRED;break;default:var n="unrecognised presence operator'"+r.str+"'";throw new e.QueryParseError(n,r.start,r.end)}var i=t.peekLexeme();if(null==i){n="expecting term or field, found nothing";throw new e.QueryParseError(n,r.start,r.end)}switch(i.type){case e.QueryLexer.FIELD:return e.QueryParser.parseField;case e.QueryLexer.TERM:return e.QueryParser.parseTerm;default:n="expecting term or field, found '"+i.type+"'";throw new e.QueryParseError(n,i.start,i.end)}}},e.QueryParser.parseField=function(t){var r=t.consumeLexeme();if(null!=r){if(-1==t.query.allFields.indexOf(r.str)){var n=t.query.allFields.map((function(e){return"'"+e+"'"})).join(", "),i="unrecognised field '"+r.str+"', possible fields: "+n;throw new e.QueryParseError(i,r.start,r.end)}t.currentClause.fields=[r.str];var s=t.peekLexeme();if(null==s){i="expecting term, found nothing";throw new e.QueryParseError(i,r.start,r.end)}switch(s.type){case e.QueryLexer.TERM:return e.QueryParser.parseTerm;default:i="expecting term, found '"+s.type+"'";throw new e.QueryParseError(i,s.start,s.end)}}},e.QueryParser.parseTerm=function(t){var r=t.consumeLexeme();if(null!=r){t.currentClause.term=r.str.toLowerCase(),-1!=r.str.indexOf("*")&&(t.currentClause.usePipeline=!1);var n=t.peekLexeme();if(null!=n)switch(n.type){case e.QueryLexer.TERM:return t.nextClause(),e.QueryParser.parseTerm;case e.QueryLexer.FIELD:return t.nextClause(),e.QueryParser.parseField;case e.QueryLexer.EDIT_DISTANCE:return e.QueryParser.parseEditDistance;case e.QueryLexer.BOOST:return e.QueryParser.parseBoost;case e.QueryLexer.PRESENCE:return t.nextClause(),e.QueryParser.parsePresence;default:var i="Unexpected lexeme type '"+n.type+"'";throw new e.QueryParseError(i,n.start,n.end)}else t.nextClause()}},e.QueryParser.parseEditDistance=function(t){var r=t.consumeLexeme();if(null!=r){var n=parseInt(r.str,10);if(isNaN(n)){var i="edit distance must be numeric";throw new e.QueryParseError(i,r.start,r.end)}t.currentClause.editDistance=n;var s=t.peekLexeme();if(null!=s)switch(s.type){case e.QueryLexer.TERM:return t.nextClause(),e.QueryParser.parseTerm;case e.QueryLexer.FIELD:return t.nextClause(),e.QueryParser.parseField;case e.QueryLexer.EDIT_DISTANCE:return e.QueryParser.parseEditDistance;case e.QueryLexer.BOOST:return e.QueryParser.parseBoost;case e.QueryLexer.PRESENCE:return t.nextClause(),e.QueryParser.parsePresence;default:i="Unexpected lexeme type '"+s.type+"'";throw new e.QueryParseError(i,s.start,s.end)}else t.nextClause()}},e.QueryParser.parseBoost=function(t){var r=t.consumeLexeme();if(null!=r){var n=parseInt(r.str,10);if(isNaN(n)){var i="boost must be numeric";throw new e.QueryParseError(i,r.start,r.end)}t.currentClause.boost=n;var s=t.peekLexeme();if(null!=s)switch(s.type){case e.QueryLexer.TERM:return t.nextClause(),e.QueryParser.parseTerm;case e.QueryLexer.FIELD:return t.nextClause(),e.QueryParser.parseField;case e.QueryLexer.EDIT_DISTANCE:return e.QueryParser.parseEditDistance;case e.QueryLexer.BOOST:return e.QueryParser.parseBoost;case e.QueryLexer.PRESENCE:return t.nextClause(),e.QueryParser.parsePresence;default:i="Unexpected lexeme type '"+s.type+"'";throw new e.QueryParseError(i,s.start,s.end)}else t.nextClause()}},function(e,t){"function"==typeof define&&define.amd?define(t):"object"==typeof exports?module.exports=t():e.lunr=t()}(this,(function(){return e}))}();
"#;

/// Search functionality JavaScript
const SEARCH_JS: &str = r#"
// Anytron Search Module
(function() {
    'use strict';

    let searchIndex = null;
    let lunrIndex = null;
    let entries = [];
    let indexLoaded = false;
    let indexLoading = false;

    const searchInput = document.getElementById('search-input');
    const searchForm = document.getElementById('search-form');
    const resultsGrid = document.getElementById('results-grid');
    const resultsInfo = document.getElementById('results-info');

    // Only initialize search on the index page
    if (!searchInput || !searchForm) return;

    // Load the search index
    async function loadIndex() {
        if (indexLoading || indexLoaded) return;
        indexLoading = true;

        try {
            resultsInfo.textContent = 'Loading search index...';
            const response = await fetch('search/index.json');
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}`);
            }
            searchIndex = await response.json();
            entries = searchIndex.entries;

            // Build lunr index
            lunrIndex = lunr(function() {
                this.ref('id');
                this.field('text');
                this.field('episode');

                const self = this;
                entries.forEach(function(entry) {
                    self.add(entry);
                });
            });

            indexLoaded = true;
            resultsInfo.textContent = `Ready to search ${entries.length} quotes`;
            console.log('Search index loaded:', entries.length, 'entries');
        } catch (error) {
            console.error('Failed to load search index:', error);
            resultsInfo.textContent = 'Failed to load search index. Please refresh the page.';
        } finally {
            indexLoading = false;
        }
    }

    // Perform search
    function performSearch(query) {
        if (!indexLoaded) {
            resultsInfo.textContent = 'Search index still loading...';
            return;
        }

        if (!query || !query.trim()) {
            resultsGrid.innerHTML = '';
            resultsInfo.textContent = `Ready to search ${entries.length} quotes`;
            return;
        }

        const startTime = performance.now();

        try {
            // Try exact search first, fall back to fuzzy
            let results = lunrIndex.search(query);
            if (results.length === 0 && query.length > 2) {
                // Try with wildcard for partial matches
                results = lunrIndex.search(query + '*');
            }

            const endTime = performance.now();
            const duration = ((endTime - startTime) / 1000).toFixed(3);

            if (results.length === 0) {
                resultsGrid.innerHTML = '';
                resultsInfo.textContent = 'No results found for "' + escapeHtml(query) + '"';
                return;
            }

            // Limit results
            const maxResults = 100;
            const limitedResults = results.slice(0, maxResults);

            resultsInfo.textContent = 'Found ' + results.length + ' results in ' + duration + 's' +
                (results.length > maxResults ? ' (showing first ' + maxResults + ')' : '');

            // Render results
            let html = '';
            for (let i = 0; i < limitedResults.length; i++) {
                const result = limitedResults[i];
                let entry = null;
                for (let j = 0; j < entries.length; j++) {
                    if (entries[j].id === result.ref) {
                        entry = entries[j];
                        break;
                    }
                }
                if (!entry) continue;

                html += '<article class="result-card">' +
                    '<a href="caption/' + entry.id + '.html" class="result-card__link">' +
                    '<img src="' + entry.thumb + '" alt="' + escapeHtml(entry.text) + '" class="result-card__image" loading="lazy">' +
                    '<div class="result-card__content">' +
                    '<p class="result-card__text">' + escapeHtml(entry.text) + '</p>' +
                    '<div class="result-card__meta">' +
                    '<span>' + entry.episode + '</span>' +
                    '<span>' + formatTimestamp(entry.timestamp) + '</span>' +
                    '</div></div></a></article>';
            }
            resultsGrid.innerHTML = html;
        } catch (error) {
            console.error('Search error:', error);
            resultsInfo.textContent = 'Search error: ' + error.message;
        }
    }

    // Format timestamp
    function formatTimestamp(ms) {
        const totalSecs = Math.floor(ms / 1000);
        const hours = Math.floor(totalSecs / 3600);
        const minutes = Math.floor((totalSecs % 3600) / 60);
        const seconds = totalSecs % 60;
        return pad(hours) + ':' + pad(minutes) + ':' + pad(seconds);
    }

    function pad(n) {
        return (n < 10 ? '0' : '') + n;
    }

    // Escape HTML
    function escapeHtml(text) {
        if (!text) return '';
        return String(text)
            .replace(/&/g, '&amp;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;')
            .replace(/"/g, '&quot;')
            .replace(/'/g, '&#39;');
    }

    // Event listeners
    searchForm.addEventListener('submit', function(e) {
        e.preventDefault();
        performSearch(searchInput.value);
    });

    // Debounced live search
    let debounceTimer = null;
    searchInput.addEventListener('input', function() {
        if (debounceTimer) clearTimeout(debounceTimer);
        debounceTimer = setTimeout(function() {
            performSearch(searchInput.value);
        }, 300);
    });

    // Load index on page load
    loadIndex();
})();
"#;

/// Meme generator JavaScript
const MEME_JS: &str = r#"
// Anytron Meme Generator Module
(function() {
    'use strict';

    // Initialize meme generator for a caption page
    window.initMemeGenerator = function(imageSrc) {
        const textArea = document.getElementById('meme-text');
        const outlineCheckbox = document.getElementById('meme-outline');
        const fontSizeSlider = document.getElementById('meme-fontsize');
        const downloadBtn = document.getElementById('meme-download');
        const copyBtn = document.getElementById('meme-copy');
        const captionOverlay = document.getElementById('caption-overlay');
        const captionText = document.getElementById('caption-text');
        const captionImage = document.getElementById('caption-image');

        if (!textArea || !downloadBtn) return;

        // Update preview text
        function updatePreview() {
            if (captionText) {
                captionText.textContent = textArea.value || '';
                captionText.style.fontSize = fontSizeSlider.value + 'px';
            }
        }

        // Word wrap helper
        function wrapText(ctx, text, maxWidth) {
            const words = text.split(' ');
            const lines = [];
            let currentLine = '';

            for (const word of words) {
                const testLine = currentLine ? currentLine + ' ' + word : word;
                const metrics = ctx.measureText(testLine);

                if (metrics.width > maxWidth && currentLine) {
                    lines.push(currentLine);
                    currentLine = word;
                } else {
                    currentLine = testLine;
                }
            }

            if (currentLine) {
                lines.push(currentLine);
            }

            return lines;
        }

        // Generate composited image as blob (shared by download and copy)
        function generateCompositeImage(callback) {
            const canvas = document.createElement('canvas');
            const ctx = canvas.getContext('2d');
            const img = new Image();
            img.crossOrigin = 'anonymous';

            img.onload = function() {
                // Set canvas size to match image
                canvas.width = img.width;
                canvas.height = img.height;

                // Draw image
                ctx.drawImage(img, 0, 0);

                // Draw text
                const text = textArea.value || '';
                if (text) {
                    const fontSize = parseInt(fontSizeSlider.value) * (img.width / captionImage.width);
                    const padding = 20;
                    const lineHeight = fontSize * 1.2;

                    ctx.font = `bold ${fontSize}px Impact, Arial, sans-serif`;
                    ctx.textAlign = 'center';
                    ctx.textBaseline = 'bottom';

                    // Word wrap
                    const maxWidth = canvas.width - (padding * 2);
                    const lines = wrapText(ctx, text, maxWidth);

                    // Calculate Y position (bottom of image)
                    let y = canvas.height - padding;

                    // Draw each line (from bottom to top)
                    for (let i = lines.length - 1; i >= 0; i--) {
                        const line = lines[i];
                        const x = canvas.width / 2;

                        if (outlineCheckbox.checked) {
                            ctx.strokeStyle = 'black';
                            ctx.lineWidth = fontSize / 10;
                            ctx.lineJoin = 'round';
                            ctx.strokeText(line, x, y);
                        }

                        ctx.fillStyle = 'white';
                        ctx.fillText(line, x, y);

                        y -= lineHeight;
                    }
                }

                callback(canvas);
            };

            img.onerror = function() {
                callback(null);
            };

            img.src = imageSrc;
        }

        // Copy image with caption to clipboard
        function copyImageWithCaption() {
            generateCompositeImage(function(canvas) {
                if (!canvas) {
                    console.error('Failed to generate image for copy');
                    return;
                }

                canvas.toBlob(function(blob) {
                    if (!blob) {
                        console.error('Failed to create blob');
                        return;
                    }

                    try {
                        navigator.clipboard.write([
                            new ClipboardItem({ 'image/png': blob })
                        ]).then(function() {
                            // Show brief feedback
                            showCopyFeedback();
                        }).catch(function(err) {
                            console.error('Failed to copy image: ', err);
                        });
                    } catch (err) {
                        console.error('Clipboard API not supported: ', err);
                    }
                }, 'image/png');
            });
        }

        // Show visual feedback when image is copied
        function showCopyFeedback() {
            const container = document.querySelector('.caption-image-container');
            if (!container) return;

            const feedback = document.createElement('div');
            feedback.textContent = 'Copied!';
            feedback.style.cssText = 'position:absolute;top:50%;left:50%;transform:translate(-50%,-50%);background:rgba(0,0,0,0.8);color:white;padding:10px 20px;border-radius:5px;font-size:18px;z-index:1000;pointer-events:none;';
            container.style.position = 'relative';
            container.appendChild(feedback);

            setTimeout(function() {
                feedback.remove();
            }, 1000);
        }

        // Event listeners
        textArea.addEventListener('input', updatePreview);
        fontSizeSlider.addEventListener('input', updatePreview);
        downloadBtn.addEventListener('click', function() {
            generateCompositeImage(function(canvas) {
                if (!canvas) {
                    alert('Failed to load image for meme generation.');
                    return;
                }
                const link = document.createElement('a');
                link.download = 'meme.png';
                link.href = canvas.toDataURL('image/png');
                link.click();
            });
        });

        // Copy button click handler
        if (copyBtn) {
            copyBtn.addEventListener('click', function() {
                copyImageWithCaption();
            });
        }

        // Handle copy event on the image
        if (captionImage) {
            captionImage.addEventListener('copy', function(e) {
                e.preventDefault();
                copyImageWithCaption();
            });

            // Also handle Ctrl+C / Cmd+C when image is focused or selected
            document.addEventListener('keydown', function(e) {
                if ((e.ctrlKey || e.metaKey) && e.key === 'c') {
                    // Check if the image or its container is in the selection
                    const selection = window.getSelection();
                    const container = document.querySelector('.caption-image-container');
                    if (container && (container.contains(document.activeElement) ||
                        (selection && selection.rangeCount > 0 && container.contains(selection.anchorNode)))) {
                        e.preventDefault();
                        copyImageWithCaption();
                    }
                }
            });

            // Handle right-click context menu copy
            captionImage.addEventListener('contextmenu', function(e) {
                // We can't override the context menu copy directly,
                // but we can add a click handler for a custom copy button
            });
        }

        // Initial preview update
        updatePreview();
    };
})();
"#;
