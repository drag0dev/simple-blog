import { Component } from '@angular/core';
import { NewPostComponent } from '../new-post/new-post.component';
import { FeedComponent } from '../feed/feed.component';

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [NewPostComponent, FeedComponent],
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss'
})
export class HomeComponent {

}
