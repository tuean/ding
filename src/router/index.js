import { createRouter, createWebHashHistory } from "vue-router";
// import Layout from '@/layout/index.vue'
// import Home from '@/views/home/Home.vue'
import Editor from "../components/Editor.vue";
import View from '../components/View.vue'
import App from '../App.vue'

const routes = [
    {
        path: "/",
        redirect: "/app/editor"
    },
    {
        name: "app",
        path: "/app",
        // component: () => import('../App.vue'),
        component: App,
        children: [
            {
                name: 'editor',
                path: "editor",
                component: () => import("../components/Editor.vue")
                // component: Editor
              },
              {
                name: 'view',
                path: "view",
                component: () => import("../components/View.vue")
                // component: View
              }
        ]
    }
];

export default createRouter({
  history: createWebHashHistory(),
  routes
});
