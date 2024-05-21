// this scripts converts the ./posts/.../readme.md into readme.bbcode . BBCode is the text syntax on microbehunter forum
// https://www.microbehunter.com/microscopy-forum/app.php/help/bbcode#f0r0
import {
    f_a_o_entry__from_s_path
}
from 'https://deno.land/x/handyhelpers@4.0.2/mod.js'

function f_s_bbcode_from_s_markdown(s_markdown, s_url_base) {
    let s = s_markdown;
    // Convert bold text
    s = s.replace(/\*\*(.*?)\*\*/g, '[b]$1[/b]');

    // Convert italic text
    s = s.replace(/\*(.*?)\*/g, '[i]$1[/i]');

    // Convert links, prepend s_url_base if it does not already start with http:// or https://
    s = s.replace(/\[([^\]]+?)\]\((.*?)\)/g, function(match, text, url) {
        if (!/^https?:\/\//i.test(url)) {
            url = s_url_base + url;
        }
        return `[url=${url}]${text}[/url]`;
    });

    // Convert images, prepend s_url_base if it does not already start with http:// or https://
    s = s.replace(/\!\[([^\]]*?)\]\((.*?)\)/g, function(match, alt, url) {
        if (!/^https?:\/\//i.test(url)) {
            url = s_url_base + url;
        }
        return `[img]${url}[/img]`;
    });

    return s;
}



let s_path_abs_file_current = import.meta.url.split("://").pop();
let s_path_abs_folder_current = s_path_abs_file_current.split('/').slice(0,-1).join('/');
console.log(s_path_abs_file_current)
// console.log(s_path_abs_folder_current)
let s_url_base = `https://raw.githubusercontent.com/jonasfrey/ruspi_microscope/main/posts`

let a_o_entry =  await f_a_o_entry__from_s_path(`${s_path_abs_folder_current}/posts`, true);
await Promise.all(
    a_o_entry.filter(o=>o.s_path_file.endsWith('.md')).map(o=>{
        let s_name_post = o.s_path_folder_parent.split('/').pop();
    Deno.readTextFile(o.s_path_file).then(s=>{
        return Deno.writeTextFile(
            o.s_path_file.split('.').slice(0,-1).join('.')+'.bbcode',
            `[comment]this file was automatically generated.[/comment]
            ${f_s_bbcode_from_s_markdown(s, `${s_url_base}/${s_name_post}/`)}
            `
        )
    });
}))

// console.log(a_o_entry);